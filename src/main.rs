

use std::{collections::HashMap, fs, time::SystemTime};
use serde_json::{Value};


static CHARSTOCHECK: &str = "abcdefghijklmnopqrstuvwxyz";


fn epoch() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_millis().try_into().unwrap(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn get_distribution(stringin: &str) -> HashMap<char, u64> {

    let mut distribution_map: HashMap<char, u64> = HashMap::new();

    for letter in stringin.to_lowercase().chars() {
        let _count = match distribution_map.get(&letter) {
            Some(_c) => _c + 1,
            _ => 1
        };
        distribution_map.insert(letter, _count);
    }

    distribution_map

}


#[derive(Debug, Clone)]
#[repr(u8)]
enum PangramStatus {
    NotEventClose = 0,
    Inperfect = 1,
    Perfect = 2
}


fn check_pangram(distin: HashMap<char, u64>) -> PangramStatus {

    let mut _count = 0;
    let mut _total = 0; 

    for _char in CHARSTOCHECK.chars() {
        if let Some(_c) = distin.get(&_char) {
            _count += 1;
            _total += _c;
        }
    }

    if _count == 26 {
        
        if _total == 26 {
            PangramStatus::Perfect        
        } else {
            PangramStatus::Inperfect
        }

    } else {
        PangramStatus::NotEventClose
    }

}


#[derive(Debug, Default)]
struct PanagramCounter {
    none: u16,
    perfect: u16,
    imperfect: u16,
}



fn main() {

    let _path = "./list.json";    
    let mut _counter = PanagramCounter::default();

    let _contents = match fs::read_to_string(_path) {
        Ok(_s) => {
            // this worked - i received something from the file...
            _s
        },
        Err(_e) => {
            // an error returned - log it then return empty string
            "".to_string()
        }
    };

    let _starttime = epoch();

    if !_contents.is_empty() {

        let _json_version: Value = match serde_json::from_str(&_contents) {
            Ok(_c) => _c,
            Err(_e) => {
                Value::Null
            }
        };

        if _json_version != Value::Null {

            let _array_of_strings = _json_version.as_array().unwrap();

            for _listitem in _array_of_strings {

                let _strval = _listitem.as_str().unwrap().to_string();
                let _dist = get_distribution(&_strval);

                match check_pangram(_dist) {
                    PangramStatus::Inperfect => {
                        _counter.imperfect += 1;
                    },
                    PangramStatus::Perfect => {
                        _counter.perfect += 1;
                    },
                    _ => {
                        _counter.none +=1;
                    }
                };

            }

        }

    }

    let _endtime = epoch();    

    println!("Result of Panagram Counter by Scolak\n");
    println!("Timetaken:                      {:>3} ms", (_endtime - _starttime));
    println!("Amount of non Panagrams:        {:>3}", _counter.none);    
    println!("Amount of imperfect Panagrams:  {:>3}", _counter.imperfect);
    println!("Amount of perfect Panagrams:    {:>3}", _counter.perfect);

}



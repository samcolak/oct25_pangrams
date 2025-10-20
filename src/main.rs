
use std::{fs, time::SystemTime};
use serde_json::{Value};


static CHARSTOCHECK: &str = "abcdefghijklmnopqrstuvwxyz";


#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(usize)]
enum PangramStatus {
    NotEvenClose = 0,
    Imperfect = 1,
    Perfect = 2
}


fn epoch() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_millis().try_into().unwrap(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}


fn get_uniques(stringin: &str) -> Vec<char> {
    let mut _r: Vec<char> = stringin.chars().collect();
    _r.sort();
    _r.dedup(); // dedup doesnt work unless the list is sorted...
    _r
}


fn get_distribution(stringin: &str) -> Vec<(char, usize)> {

    let _lower = stringin.to_lowercase();
    let _uniques = get_uniques(&_lower);
    let _allchars: Vec<char> = _lower.chars().collect();

    let mut _frequency: Vec<(char, usize)> = _uniques.into_iter()
        .map(|x| (x, _allchars.iter().filter(|&n| *n == x).count()))
        .collect();

    _frequency
}


fn check_pangram(distin: Vec<(char, usize)>) -> PangramStatus {
    let mut _count = 0;
    let mut _total = 0; 
    for (_char, _counter) in distin.iter() {
        if CHARSTOCHECK.contains(*_char) {
            _count += 1;
            _total += _counter;            
        }
    }
    if _count != 26 {
        PangramStatus::NotEvenClose        
    } else if _total == 26 {
        PangramStatus::Perfect        
    } else {
        PangramStatus::Imperfect
    }
}


fn main() {

    let _path = "./list.json";    
 
    let _contents = match fs::read_to_string(_path) {
        Ok(_s) => { // this worked - i received something from the file...
            _s
        },
        Err(_e) => { // an error returned - log it then return empty string
            "".to_string()
        }
    };

    if !_contents.is_empty() {

        let _json_version: Value = match serde_json::from_str(&_contents) {
            Ok(_c) => _c,
            Err(_e) => {
                Value::Null
            }
        };
 
        if _json_version != Value::Null {

            let mut _counters: [u16; 3] = [0; 3];               

            let _array_of_strings = _json_version.as_array().unwrap();
            let _starttime = epoch();

            // start the processing loop

            for _listitem in _array_of_strings {

                if let Some(_v) = _listitem.as_str() {
                    let _dist = get_distribution(_v);
                    let _state = check_pangram(_dist);
                    _counters[_state as usize] += 1;
                }

            }

            // finished...

            let _endtime = epoch();    

            println!("Result of Panagram Counter by Scolak\n");
            println!("Time taken:                     {:>3} ms", (_endtime - _starttime));
            println!("Amount of non Panagrams:        {:>3}", _counters[PangramStatus::NotEvenClose as usize]);    
            println!("Amount of imperfect Panagrams:  {:>3}", _counters[PangramStatus::Imperfect as usize]);
            println!("Amount of perfect Panagrams:    {:>3}", _counters[PangramStatus::Perfect as usize]);

        } else {

            println!("No list to process");

        }

    } else {

        println!("Failed to read file - aborting");

    }

}



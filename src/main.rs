

use std::{collections::HashMap, fs};
use serde_json::{Value};


static CHARSTOCHECK: &str = "abcdefghijklmnopqrstuvwxyz";


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

    let mut count = 0;
    let mut total = 0; 

    for _char in CHARSTOCHECK.chars() {
        if let Some(_c) = distin.get(&_char) {
            count += 1;
            total += _c;
        }
    }

    if count == 26 {
        
        if total == 26 {
            PangramStatus::Perfect        
        } else {
            PangramStatus::Inperfect
        }

    } else {
        PangramStatus::NotEventClose
    }

}





fn main() {

    let file_path = "./list.json";

    let contents = match fs::read_to_string(file_path) {
        Ok(_s) => {
            // this worked - i received something from the file...
            _s
        },
        Err(_e) => {
            // an error returned - log it then return empty string
            "".to_string()
        }
    };

    if !contents.is_empty() {

        let _json_version: Value = match serde_json::from_str(&contents) {
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
                    PangramStatus::Inperfect => println!("Inperfect = {_strval}"),
                    PangramStatus::Perfect => println!("** Perfect = {_strval}"),
                    _ => {}
                };

            }

        }

    }

}



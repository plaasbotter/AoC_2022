use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec;

pub fn read_file_lines(path: impl AsRef<std::path::Path>) -> Vec<String>{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut return_value: Vec<String> = vec![];
    for line in reader.lines() {
        match line {
            Ok(v) => {return_value.push(v)},
            Err(e) => {println!("{}", e)},
        }
    }
    return return_value;
}

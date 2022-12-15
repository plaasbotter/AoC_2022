use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::vec;

pub fn read_file_lines(path: impl AsRef<std::path::Path>) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut return_value: Vec<String> = vec![];
    for line in reader.lines() {
        match line {
            Ok(v) => return_value.push(v),
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    return return_value;
}

pub fn _create_all_needed_files() {
    let paths = fs::read_dir("./data").unwrap();
    let mut all_data: HashSet<String> = HashSet::new();
    for entry in paths {
        let file_name = entry.unwrap().file_name();
        all_data.insert(file_name.to_str().unwrap().to_string());
    }
    for i in 1..26 {
        for j in 1..3 {
            let temp_data: String = format!("day_{}_{}.txt", i, j);
            if all_data.contains(&temp_data) == false {
                File::create(format!("./data/{}", temp_data)).unwrap();
            }
        }
    }
    let paths = fs::read_dir("./src").unwrap();
    let mut all_data: HashSet<String> = HashSet::new();
    for entry in paths {
        let file_name = entry.unwrap().file_name();
        all_data.insert(file_name.to_str().unwrap().to_string());
    }
    for i in 1..26 {
        let temp_data: String = format!("day_{}.rs", i);
        if all_data.contains(&temp_data) == false {
            let mut f = File::create(format!("./src/{}", temp_data)).unwrap();
            f.write_all(b"pub fn run_1(input: &Vec<String>) {println!(\"Answer = {}\",0);} \n pub fn run_2(input: &Vec<String>) {println!(\"Answer = {}\",0);}").unwrap();
            f.sync_all().unwrap();
        }
    }
}

pub fn _generate_mod_commands() {
    for i in 1..26 {
        println!("//mod day_{};", i);
    }
}

pub fn _check_size(temp: isize, largest: &mut isize, smallest: &mut isize) {
    if temp > *largest {
        *largest = temp;
    }
    if temp < *smallest {
        *smallest = temp;
    }
}

pub fn _print_bool_grid(grid: &Vec<Vec<bool>>) {
    for y in grid {
        for x in y {
            if *x == false {
                print!(".")
            } else {
                print!("#");
            }
        }
        println!();
    }
}

pub fn _print_u8_grid(grid: &Vec<Vec<u8>>) {
    for y in grid {
        for x in y {
            print!("{}", x);
        }
        println!();
    }
}

pub fn _generate_run_commands() {
    for i in 1..26 {
        println!("//println!(\"Day{}\");", i);
        println!(
            "//let day_{0}_data: Vec<String> = utils::read_file_lines(\"./data/day_{0}_1.txt\");",
            i
        );
        println!("//day_{0}::run_1(&day_{0}_data);", i);
        println!("//day_{0}::run_2(&day_{0}_data);", i);
        println!("////");
    }
}

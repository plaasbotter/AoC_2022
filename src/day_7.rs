use std::collections::HashMap;

use crate::utils;

#[derive(Debug)]
struct FileSystem {
    //Use atomics for multi threaded
    atom: usize,
    directory_system: HashMap<usize, Directory>,
}

impl FileSystem {
    fn add_directory(&mut self, name: String, parent: Option<usize>) -> usize {
        self.atom += 1;
        self.directory_system.insert(
            self.atom.clone(),
            Directory::new(self.atom.clone(), name, parent),
        );
        return self.atom.clone();
    }
    fn new() -> FileSystem {
        return FileSystem {
            atom: 0,
            directory_system: HashMap::new(),
        };
    }
}

#[derive(Debug)]
struct Directory {
    _idx: usize,
    _name: String,
    size: usize,
    parent: Option<usize>,
    children: HashMap<String, usize>,
    files: HashMap<String, File>,
}

impl Directory {
    fn new(idx: usize, name: String, parent: Option<usize>) -> Directory {
        return Directory {
            _idx: idx,
            _name: name,
            parent: parent,
            children: HashMap::new(),
            files: HashMap::new(),
            size: 0,
        };
    }
    fn add_file(&mut self, file_name: String, file_size: usize) {
        self.files
            .insert(file_name.clone(), File::new(file_name.clone(), file_size));
    }
}

#[derive(Debug)]
struct File {
    _file_name: String,
    _file_size: usize,
}

impl File {
    fn new(file_name: String, file_size: usize) -> File {
        return File {
            _file_name: file_name,
            _file_size: file_size,
        };
    }
}

pub fn run_1() {
    let all_commands: Vec<String> = utils::read_file_lines("./data/day_7_1.txt");
    let file_system: FileSystem = get_file_system(all_commands);
    let current_directory: usize = 1;
    let total = get_file_size_under(&file_system, current_directory, 100000 as usize);
    println!("Answer = {}", total);
}

fn get_file_size_under(
    file_system: &FileSystem,
    current_directory: usize,
    maximum: usize,
) -> usize {
    let mut total: usize = 0;
    let current = file_system
        .directory_system
        .get(&current_directory)
        .unwrap();
    for (_key, value) in &current.children {
        total += get_file_size_under(file_system, (*value).clone(), maximum);
    }
    if current.size < maximum {
        total += current.size;
    }
    return total;
}

fn get_file_system(all_commands: Vec<String>) -> FileSystem {
    let mut file_system: FileSystem = FileSystem::new();
    let mut current_node = file_system.add_directory(String::from("root"), None);
    for command in all_commands {
        let splitted_command: Vec<&str> = command.split(' ').collect::<Vec<_>>();
        if splitted_command.len() >= 2 {
            match splitted_command[0] {
                "$" => {
                    if splitted_command[1] == "cd" {
                        if splitted_command[2] == "/" {
                            current_node = 1;
                        } else if splitted_command[2] == ".." {
                            match file_system
                                .directory_system
                                .get(&current_node)
                                .unwrap()
                                .parent
                            {
                                Some(v) => current_node = v,
                                None => {
                                    println!("Could not find parent: {}", splitted_command[2]);
                                    panic!();
                                }
                            }
                        } else {
                            match file_system
                                .directory_system
                                .get(&current_node)
                                .unwrap()
                                .children
                                .get(splitted_command[2])
                            {
                                Some(v) => current_node = *v,
                                None => {
                                    println!("Could not find filename: {}", splitted_command[2])
                                }
                            }
                        }
                    }
                }
                "dir" => {
                    let file_name: String = splitted_command[1].to_string();
                    let new_file = file_system.add_directory(file_name.clone(), Some(current_node));
                    file_system
                        .directory_system
                        .get_mut(&current_node)
                        .unwrap()
                        .children
                        .insert(file_name.clone(), new_file);
                }
                _val => {
                    let size: usize = splitted_command[0].parse::<usize>().unwrap();
                    let name: String = splitted_command[1].to_string();
                    file_system
                        .directory_system
                        .get_mut(&current_node)
                        .unwrap()
                        .add_file(name, size);
                    update_file_size(&mut file_system, &current_node, &size)
                }
            }
        }
    }
    return file_system;
}

fn update_file_size(file_system: &mut FileSystem, current_node: &usize, size: &usize) {
    let temp_directory = file_system.directory_system.get_mut(current_node).unwrap();
    temp_directory.size += *size;
    match temp_directory.parent {
        Some(v) => update_file_size(file_system, &v, size),
        None => {}
    }
}

pub fn run_2() {
    let all_commands: Vec<String> = utils::read_file_lines("./data/day_7_1.txt");
    let file_system: FileSystem = get_file_system(all_commands);
    let current_directory: usize = 1;
    let temp_max_size = 30000000
        - (70000000
            - file_system
                .directory_system
                .get(&current_directory)
                .unwrap()
                .size as isize);
    if temp_max_size > 0 {
        let max_size = temp_max_size as usize;
        let total = get_small_enough_folder(&file_system, current_directory, max_size);
        println!("Answer = {}", total);
    }
}

fn get_small_enough_folder(
    file_system: &FileSystem,
    current_directory: usize,
    maximum: usize,
) -> usize {
    let mut total: usize = 70000000;
    let current = file_system
        .directory_system
        .get(&current_directory)
        .unwrap();
    for (_key, value) in &current.children {
        let new_size = get_small_enough_folder(file_system, (*value).clone(), maximum);
        if new_size > 0 && new_size < total {
            total = new_size.clone();
        }
    }
    if current.size > maximum && current.size < total {
        total = current.size;
    }
    return total;
}

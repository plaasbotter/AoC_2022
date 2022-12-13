use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Instruction {
    Add,
    Mul,
    Unknown,
}

#[derive(Clone, Debug)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    divistion: usize,
    true_direction: usize,
    false_direction: usize,
    alter_value: Option<usize>,
    insturction: Instruction,
    activity_counter: usize,
}

#[derive(Clone, Debug)]
struct Monkey2 {
    id: usize,
    items: Vec<u128>,
    divistion: u128,
    true_direction: usize,
    false_direction: usize,
    alter_value: Option<u128>,
    insturction: Instruction,
    activity_counter: usize,
}

impl Monkey2 {
    fn new() -> Monkey2 {
        return Monkey2 {
            id: 0,
            items: vec![],
            divistion: 0,
            true_direction: 0,
            false_direction: 0,
            alter_value: None,
            insturction: Instruction::Unknown,
            activity_counter: 0,
        };
    }
}

impl Monkey {
    fn new() -> Monkey {
        return Monkey {
            id: 0,
            items: vec![],
            divistion: 0,
            true_direction: 0,
            false_direction: 0,
            alter_value: None,
            insturction: Instruction::Unknown,
            activity_counter: 0,
        };
    }
}

pub fn run_1(input: &Vec<String>) {
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
    let mut temp_monkey = Monkey::new();

    for line in input {
        if line.is_empty() {
            monkeys.insert(temp_monkey.id, temp_monkey.clone());
            temp_monkey = Monkey::new();
        } else if line.starts_with("Monkey") {
            let new_line = line.clone().replace(":", "");
            let splitted: Vec<&str> = new_line.split(' ').collect();
            temp_monkey.id = splitted[1].parse::<usize>().unwrap();
        } else if line.starts_with("  Starting items") {
            let new_line = line.clone().replace(",", "");
            let splitted: Vec<&str> = new_line.split(' ').collect();
            for i in 4..splitted.len() {
                temp_monkey
                    .items
                    .push(splitted[i].parse::<usize>().unwrap());
            }
        } else if line.starts_with("  Operation") {
            let splitted: Vec<&str> = line.split(' ').collect();
            match splitted[6] {
                "*" => temp_monkey.insturction = Instruction::Mul,
                "+" => temp_monkey.insturction = Instruction::Add,
                val => {
                    println!("Unknown instruction: {}", val);
                }
            }
            match splitted[7] {
                "old" => temp_monkey.alter_value = None,
                _val => temp_monkey.alter_value = Some(splitted[7].parse::<usize>().unwrap()),
            }
        } else if line.starts_with("  Test") {
            let splitted: Vec<&str> = line.split(' ').collect();
            temp_monkey.divistion = splitted[5].parse::<usize>().unwrap();
        } else if line.starts_with("    If true") {
            let splitted: Vec<&str> = line.split(' ').collect();
            temp_monkey.true_direction = splitted[9].parse::<usize>().unwrap();
        } else if line.starts_with("    If false") {
            let splitted: Vec<&str> = line.split(' ').collect();
            temp_monkey.false_direction = splitted[9].parse::<usize>().unwrap();
        }
    }
    monkeys.insert(temp_monkey.id, temp_monkey.clone());

    for _k in 0..20 {
        let mut temp_val: usize;
        let mut temp_instruction: Vec<(usize, usize)> = vec![];
        for i in 0..monkeys.len() {
            {
                let mut currentmonkey: &mut Monkey = monkeys.get_mut(&i).unwrap();
                while currentmonkey.items.len() > 0 {
                    let mut item = currentmonkey.items.pop().unwrap();
                    if currentmonkey.alter_value == None {
                        temp_val = item.clone();
                    } else {
                        temp_val = currentmonkey.alter_value.unwrap();
                    }
                    match currentmonkey.insturction {
                        Instruction::Add => item += temp_val,
                        Instruction::Mul => item *= temp_val,
                        Instruction::Unknown => println!("Unknown command!"),
                    }
                    item = item / 3;
                    if item % currentmonkey.divistion == 0 {
                        temp_instruction.push((item, currentmonkey.true_direction.clone()));
                    } else {
                        temp_instruction.push((item, currentmonkey.false_direction.clone()));
                    }
                    currentmonkey.activity_counter += 1;
                }
            }

            while temp_instruction.len() > 0 {
                let temp_temp_instruction = temp_instruction.pop().unwrap();
                monkeys
                    .get_mut(&temp_temp_instruction.1)
                    .unwrap()
                    .items
                    .push(temp_temp_instruction.0);
            }
        }
    }
    let mut largest_a = 0;
    let mut largest_b = 0;
    let mut _largest_c = 0;
    for i in 0..monkeys.len() {
        {
            let current_monkey = monkeys.get(&i).unwrap();
            if current_monkey.activity_counter > largest_b {
                largest_b = current_monkey.activity_counter.clone();
            }
        }
        if largest_b > largest_a{
            _largest_c = largest_a.clone();
            largest_a = largest_b.clone();
            largest_b = _largest_c.clone();
        }
    }
    println!("Answer = {}", largest_a * largest_b);
}

pub fn run_2(input: &Vec<String>) {
    let mut monkeys: HashMap<usize, Monkey2> = HashMap::new();
    let mut temp_monkey = Monkey2::new();
    let mudolo: u128 = 2*3*5*7*11*13*17*19;
    for line in input {
        if line.is_empty() {
            monkeys.insert(temp_monkey.id, temp_monkey.clone());
            temp_monkey = Monkey2::new();
        } else if line.starts_with("Monkey") {
            let new_line = line.clone().replace(":", "");
            let splitted: Vec<&str> = new_line.split(' ').collect();
            temp_monkey.id = splitted[1].parse::<usize>().unwrap();
        } else if line.starts_with("  Starting items") {
            let new_line = line.clone().replace(",", "");
            let splitted: Vec<&str> = new_line.split(' ').collect();
            for i in 4..splitted.len() {
                temp_monkey
                    .items
                    .push(splitted[i].parse::<u128>().unwrap());
            }
        } else if line.starts_with("  Operation") {
            let splitted: Vec<&str> = line.split(' ').collect();
            match splitted[6] {
                "*" => temp_monkey.insturction = Instruction::Mul,
                "+" => temp_monkey.insturction = Instruction::Add,
                val => {
                    println!("Unknown instruction: {}", val);
                }
            }
            match splitted[7] {
                "old" => temp_monkey.alter_value = None,
                _val => temp_monkey.alter_value = Some(splitted[7].parse::<u128>().unwrap()),
            }
        } else if line.starts_with("  Test") {
            let splitted: Vec<&str> = line.split(' ').collect();
            temp_monkey.divistion = splitted[5].parse::<u128>().unwrap();
        } else if line.starts_with("    If true") {
            let splitted: Vec<&str> = line.split(' ').collect();
            temp_monkey.true_direction = splitted[9].parse::<usize>().unwrap();
        } else if line.starts_with("    If false") {
            let splitted: Vec<&str> = line.split(' ').collect();
            temp_monkey.false_direction = splitted[9].parse::<usize>().unwrap();
        }
    }
    monkeys.insert(temp_monkey.id, temp_monkey.clone());

    for _k in 0..10000 {
        let mut temp_val: u128;
        let mut temp_instruction: Vec<(u128, usize)> = vec![];
        for i in 0..monkeys.len() {
            {
                let mut currentmonkey: &mut Monkey2 = monkeys.get_mut(&i).unwrap();
                while currentmonkey.items.len() > 0 {
                    let mut item = currentmonkey.items.pop().unwrap();
                    if currentmonkey.alter_value == None {
                        temp_val = item.clone();
                    } else {
                        temp_val = currentmonkey.alter_value.unwrap();
                    }
                    match currentmonkey.insturction {
                        Instruction::Add => item += temp_val,
                        Instruction::Mul => item *= temp_val,
                        Instruction::Unknown => println!("Unknown command!"),
                    }
                    item = item % mudolo;
                    //item = item / 3;
                    if item % currentmonkey.divistion == 0 {
                        temp_instruction.push((item, currentmonkey.true_direction.clone()));
                        //temp_instruction.push((item / currentmonkey.divistion.clone(), currentmonkey.true_direction.clone()));
                    } else {
                        temp_instruction.push((item, currentmonkey.false_direction.clone()));
                       //temp_instruction.push((currentmonkey.divistion.clone(), currentmonkey.false_direction.clone()));
                    }
                    currentmonkey.activity_counter += 1;
                }
            }

            while temp_instruction.len() > 0 {
                let temp_temp_instruction = temp_instruction.pop().unwrap();
                monkeys
                    .get_mut(&temp_temp_instruction.1)
                    .unwrap()
                    .items
                    .push(temp_temp_instruction.0);
            }
        }
    }
    let mut largest_a = 0;
    let mut largest_b = 0;
    let mut _largest_c = 0;
    for i in 0..monkeys.len() {
        {
            let current_monkey = monkeys.get(&i).unwrap();
            if current_monkey.activity_counter > largest_b {
                largest_b = current_monkey.activity_counter.clone();
            }
        }
        if largest_b > largest_a{
            _largest_c = largest_a.clone();
            largest_a = largest_b.clone();
            largest_b = _largest_c.clone();
        }
    }
    println!("Answer = {}", largest_a * largest_b);
}

#[derive(Debug, Clone)]
struct Item {
    list: Vec<Item>,
    val: usize,
    is_empty: bool,
}

#[derive(Clone)]
enum Comparison {
    Correct,
    Wrong,
    Continue,
    Unknown,
}

impl Item {
    fn new() -> Item {
        return Item {
            list: vec![],
            val: 0,
            is_empty: true,
        };
    }
}

pub fn run_1(input: &Vec<String>) {
    let mut row_counter: usize = 0;
    let mut correct_counter: usize = 0;
    while row_counter < input.len() - 1 {
        let row_1 = input[row_counter].clone();
        let row_2 = input[row_counter + 1].clone();
        let mut is_correct: bool = false;
        let comparrison = compare_pairs(row_1, row_2);
        match comparrison {
            Comparison::Correct => is_correct = true,
            Comparison::Wrong => is_correct = false,
            Comparison::Continue => println!("Continue possible error"),
            Comparison::Unknown => println!("Uknown possible error"),
        }
        if is_correct == true {
            correct_counter += (row_counter / 3) + 1;
        }
        row_counter += 3;
    }
    println!("Answer = {}", correct_counter);
}

fn compare_pairs(row_1: String, row_2: String) -> Comparison {
    let mut left_items: Item = Item::new();
    parse_items(&mut left_items, row_1.as_str(), 1, row_1.len() - 1);
    let mut right_items: Item = Item::new();
    parse_items(&mut right_items, row_2.as_str(), 1, row_2.len() - 1);
    return compare_pair(&left_items, &right_items);
}

fn compare_pair(left_items: &Item, right_items: &Item) -> Comparison {
    let mut return_value = Comparison::Unknown;
    if left_items.is_empty == true && right_items.is_empty == false {
        return Comparison::Correct;
    } else if left_items.is_empty == false && right_items.is_empty == true {
        return Comparison::Wrong;
    } else {
        let left_is_number: u8 = (left_items.list.len() == 0) as u8;
        let right_is_number: u8 = (right_items.list.len() == 0) as u8;
        let total: u8 = left_is_number + right_is_number;
        if total == 2 {
            match &left_items.val.cmp(&right_items.val) {
                std::cmp::Ordering::Less => {
                    return_value = Comparison::Correct;
                }
                std::cmp::Ordering::Equal => {
                    return_value = Comparison::Continue;
                }
                std::cmp::Ordering::Greater => {
                    return_value = Comparison::Wrong;
                }
            }
        } else if total == 1 {
            if left_is_number == 1 {
                let new_left: Item = Item {
                    list: vec![left_items.clone()],
                    val: 0,
                    is_empty: false,
                };
                return_value = compare_pair(&new_left, &right_items);
            } else {
                let new_right: Item = Item {
                    list: vec![right_items.clone()],
                    val: 0,
                    is_empty: false,
                };
                return_value = compare_pair(&left_items, &new_right);
            }
        } else {
            for i in 0..right_items.list.len() {
                if i >= left_items.list.len() {
                    return_value = Comparison::Correct;
                    break;
                }
                return_value = compare_pair(&left_items.list[i], &right_items.list[i]);
                let clone = return_value.clone();
                if clone as u8 != Comparison::Continue as u8 {
                    return return_value;
                }
            }
            if left_items.list.len() > right_items.list.len() {
                return_value = Comparison::Wrong;
            }
        }
    }
    return return_value;
}

fn parse_items(items: &mut Item, row: &str, left_target: usize, right_target: usize) {
    let chars: Vec<char> = row.chars().collect();
    let mut start_section: usize = left_target;
    let mut start_list: usize = 0;
    let mut counter_list: usize = 0;
    let mut started_list: bool = false;
    for i in left_target..right_target {
        if i == right_target - 1 && started_list == false {
            let mut temp_item: Item = Item::new();
            match (&row[start_section..right_target]).parse::<usize>() {
                Ok(v) => {
                    temp_item.val = v;
                    temp_item.is_empty = false;
                }
                Err(e) => println!("Could not parse {:?}", e),
            }
            items.list.push(temp_item.clone());
        } else if chars[i] == ',' && counter_list == 0 {
            if started_list == true {
                started_list = false;
            } else {
                parse_items(items, row, start_section, i);
            }
            start_section = i + 1;
        } else if chars[i] == '[' {
            if started_list == false {
                started_list = true;
                start_list = i;
            }
            counter_list += 1;
        } else if chars[i] == ']' {
            counter_list -= 1;
            if counter_list == 0 {
                let mut temp_item = Item::new();
                parse_items(&mut temp_item, row, start_list + 1, i);
                items.list.push(temp_item);
            }
        }
    }
    if items.list.len() > 0 {
        items.is_empty = false;
    }
}

pub fn run_2(input: &Vec<String>) {
    let mut pos_two: usize = 0;
    let mut pos_six: usize = 0;
    let mut all_items: Vec<Item> = vec![];
    {
        for line in input {
            if line.is_empty() == false {
                let mut temp_item: Item = Item::new();
                parse_items(&mut temp_item, line.as_str(), 1, line.len() - 1);
                all_items.push(temp_item.clone())
            }
        }
    }
    {
        let line = String::from("[[6]]");
        let mut temp_item: Item = Item::new();
        temp_item.val = usize::MAX; //MARKER TO IDENTIFY
        parse_items(&mut temp_item, line.as_str(), 1, line.len() - 1);
        all_items.push(temp_item.clone())
    }
    {
        let line = String::from("[[2]]");
        let mut temp_item: Item = Item::new();
        temp_item.val = usize::MAX - 1; //MARKER TO IDENTIFY
        parse_items(&mut temp_item, line.as_str(), 1, line.len() - 1);
        all_items.push(temp_item.clone())
    }
    let mut in_loop = true;
    while in_loop {
        in_loop = false;
        for i in 0..all_items.len() - 1 {
            let comparrison = compare_pair(&all_items[i], &all_items[i + 1]) as u8;
            if comparrison != Comparison::Correct as u8 {
                all_items.swap(i, i + 1);
                if all_items[i].val == usize::MAX - 1 {
                    pos_two = i;
                }
                if all_items[i + 1].val == usize::MAX - 1 {
                    pos_two = i + 1;
                }
                if all_items[i].val == usize::MAX {
                    pos_six = i;
                }
                if all_items[i + 1].val == usize::MAX {
                    pos_six = i + 1
                }
                in_loop = true;
                break;
            }
        }
    }
    println!("Answer = {}", (pos_six+1) * (pos_two+1));
}

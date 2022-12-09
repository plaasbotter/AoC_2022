use std::collections::HashSet;

pub fn run_1(input: &Vec<String>) {
    let mut total: usize = 0;
    for pack in input {
        let length = pack.len() / 2;
        let mut val_left: u8 = 0;
        'outer: for pos_left in 0..length {
            val_left = pack.chars().nth(pos_left).unwrap() as u8;
            for pos_right in length..length * 2 {
                if pack.chars().nth(pos_right).unwrap() as u8 == val_left {
                    break 'outer;
                }
            }
        }
        if val_left < 97 {
            val_left += 58;
        }
        val_left -= 96;
        total += val_left as usize;
    }
    println!("Answer {}", total);
}

pub fn run_2(input: &Vec<String>) {
    let mut total: usize = 0;
    let mut current: usize = 0;
    while current < input.len() {
        let length_pack_1 = input[current].len();
        let mut val_pack_1: u8 = 0;
        let mut searched_values_pack_1: HashSet<u8> = HashSet::new();
        'outer: for pos_pack_1 in 0..length_pack_1 {
            val_pack_1 = input[current].chars().nth(pos_pack_1).unwrap() as u8;
            if searched_values_pack_1.contains(&val_pack_1) == false {
                searched_values_pack_1.insert(val_pack_1);
                let length_pack_2 = input[current + 1].len();
                let mut val_pack_2: u8;
                'inner: for pos_pack_2 in 0..length_pack_2 {
                    val_pack_2 = input[current + 1].chars().nth(pos_pack_2).unwrap() as u8;
                    if val_pack_1 == val_pack_2 {
                        let length_pack_3 = input[current + 2].len();
                        let mut val_pack_3: u8;
                        for pos_pack_3 in 0..length_pack_3 {
                            val_pack_3 =
                            input[current + 2].chars().nth(pos_pack_3).unwrap() as u8;
                            if val_pack_2 == val_pack_3 {
                                break 'outer;
                            }
                        }
                        break 'inner;
                    }
                }
            }
        }
        if val_pack_1 < 97 {
            val_pack_1 += 58;
        }
        val_pack_1 -= 96;
        total += val_pack_1 as usize;
        current += 3;
    }
    println!("Answer {}", total);
}

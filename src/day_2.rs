use crate::utils;

pub fn run_1() {
    let all_plays: Vec<String> =
        utils::read_file_lines("./data/day_2_1.txt");
    let abc_start: isize = -66;
    let xyz_start: isize = -89;
    let mut move_opponent: isize = 0;
    let mut move_self: isize = 0;
    let mut total_score: isize = 0;
    let mut do_calc: bool;
    for play in all_plays {
        do_calc = true;
        match play.chars().nth(0) {
            Some(v) => {
                move_opponent = abc_start + (v as isize);
            }
            None => {
                do_calc = false;
            }
        };
        match play.chars().nth(2) {
            Some(v) => {
                move_self = xyz_start + (v as isize);
            }
            None => {
                do_calc = false;
            }
        };
        if do_calc == true {
            let difference = move_opponent - move_self;
            total_score += move_self + 2;
            if difference == 0 {
                total_score += 3;
            } else if difference == -1 || difference == 2 {
                total_score += 6;
            }
        }
    }
    println!("Answer: {}", total_score);
}

pub fn run_2() {
    let all_plays: Vec<String> =
        utils::read_file_lines("./data/day_2_1.txt");
    let abc_start: isize = -64;
    let xyz_start: isize = -88;
    let mut move_opponent: isize = 0;
    let mut move_self: isize = 0;
    let mut total_score: isize = 0;
    let mut do_calc: bool;
    for play in all_plays {
        do_calc = true;
        match play.chars().nth(0) {
            Some(v) => {
                move_opponent = abc_start + (v as isize);
            }
            None => {
                do_calc = false;
            }
        };
        match play.chars().nth(2) {
            Some(v) => {
                move_self = xyz_start + (v as isize);
            }
            None => {
                do_calc = false;
            }
        };
        if do_calc == true {
            total_score += move_self * 3;
            total_score += ((move_opponent + move_self + 1) % 3) + 1
        }
    }
    println!("Answer: {}", total_score);
}

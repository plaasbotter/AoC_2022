pub fn run_1(input: &Vec<String>) {
    let mut total: usize = 0;
    let mut largest: usize = 0;
    for calorie in input {
        if calorie.is_empty() {
            reset_total(&mut total, &mut largest);
        } else {
            match calorie.parse::<usize>() {
                Ok(v) => total += v,
                Err(e) => println!("{}", e),
            }
        }
    }
    reset_total(&mut total, &mut largest);
    println!("Answer: {}", largest);
}

pub fn run_2(input: &Vec<String>) {
    let mut total: usize = 0;
    let mut largest_1: usize = 0;
    let mut largest_2: usize = 0;
    let mut largest_3: usize = 0;
    for calorie in input {
        if calorie.is_empty() {
            reset_total_top_three(&mut total, &mut largest_1, &mut largest_2, &mut largest_3);
        } else {
            match calorie.parse::<usize>() {
                Ok(v) => total += v,
                Err(e) => println!("{}", e),
            }
        }
    }
    reset_total_top_three(&mut total, &mut largest_1, &mut largest_2, &mut largest_3);
    println!("Answer: {}", largest_1 + largest_2 + largest_3);
}

fn reset_total_top_three(
    total: &mut usize,
    largest_1: &mut usize,
    largest_2: &mut usize,
    largest_3: &mut usize,
) {
    if *total > *largest_3 {
        *largest_3 = *total;
        if *largest_3 > *largest_2 {
            *total = *largest_2;
            *largest_2 = *largest_3;
            *largest_3 = *total;
            if *largest_2 > *largest_1 {
                *total = *largest_1;
                *largest_1 = *largest_2;
                *largest_2 = *total;
            }
        }
    }
    *total = 0;
}

fn reset_total(total: &mut usize, largest: &mut usize) {
    if *total > *largest {
        *largest = *total;
    }
    *total = 0;
}

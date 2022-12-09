use std::collections::VecDeque;

pub fn run_1(input: &Vec<String>) {
    let stacks_amount: usize = (input[0].len() + 1) / 4;
    let mut begin_instructions: bool = false;
    let mut matrix: Vec<VecDeque<char>> = vec![];
    for _i in 0..stacks_amount{
        matrix.push(VecDeque::new());
    }
    for line in input{
        if line.is_empty(){
            begin_instructions = true;
        }
        else if begin_instructions == false{
            if line.as_bytes()[1] != b'1'{
                for i in 0..stacks_amount{
                    let localation: usize = (i + 1) + (i * 3);
                    let new_char: char = line.as_bytes()[localation] as char;
                    //println!("{} {}",localation, new_char);
                    if new_char != ' '{
                        matrix[i].push_front(new_char);
                    }
                }
            }
        }
        else {
            let splitted_line: Vec<&str> = line.split(' ').collect();
            let a = splitted_line[1].parse::<usize>().unwrap();
            let b = splitted_line[3].parse::<usize>().unwrap() - 1;
            let c = splitted_line[5].parse::<usize>().unwrap() - 1;
            for _i in 0..a{
                let temp_box = matrix[b].pop_back().unwrap();
                matrix[c].push_back(temp_box);
            }
        }
    }
    let mut answer: String = String::new();
    for i in 0..stacks_amount{
        answer.push(matrix[i].pop_back().unwrap());
    }
    println!("Answer = {}", answer);
}

pub fn run_2(input: &Vec<String>) {
    let stacks_amount: usize = (input[0].len() + 1) / 4;
    let mut begin_instructions: bool = false;
    let mut matrix: Vec<VecDeque<char>> = vec![];
    for _i in 0..stacks_amount{
        matrix.push(VecDeque::new());
    }
    for line in input{
        if line.is_empty(){
            begin_instructions = true;
            //println!("{:#?}", matrix);
        }
        else if begin_instructions == false{
            if line.as_bytes()[1] != b'1'{
                for i in 0..stacks_amount{
                    let localation: usize = (i + 1) + (i * 3);
                    let new_char: char = line.as_bytes()[localation] as char;
                    //println!("{} {}",localation, new_char);
                    if new_char != ' '{
                        matrix[i].push_front(new_char);
                    }
                }
            }
        }
        else {
            let splitted_line: Vec<&str> = line.split(' ').collect();
            let a = splitted_line[1].parse::<usize>().unwrap();
            let b = splitted_line[3].parse::<usize>().unwrap() - 1;
            let c = splitted_line[5].parse::<usize>().unwrap() - 1;
            let mut temp_stack: VecDeque<char> = VecDeque::new();
            for _i in 0..a{
                let temp_box = matrix[b].pop_back().unwrap();
                temp_stack.push_front(temp_box);
            }
            while temp_stack.len() > 0{
                let temp_box = temp_stack.pop_front().unwrap();
                matrix[c].push_back(temp_box);
            }
            //println!("{:#?}", matrix);
        }
    }
    let mut answer: String = String::new();
    for i in 0..stacks_amount{
        answer.push(matrix[i].pop_back().unwrap());
    }
    println!("Answer = {}", answer);
}

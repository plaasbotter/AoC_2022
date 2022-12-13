pub fn run_1(input: &Vec<String>) {
    let mut register_x: i32 = 1;
    let mut register_a: i32 = 0;
    let mut total: i32 = 0;
    let mut clock = 0;
    let mut instruction_counter = 0;
    while instruction_counter < input.len() {
        clock += 1;
        if clock % 20 == 0 {
            if clock == 20 || (clock - 20) % 40 == 0 {
                total += clock * register_x;
            }
        }
        if register_a != 0 {
            register_x += register_a;
            register_a = 0;
        } else {
            let splitted_instructions: Vec<&str> = input[instruction_counter].split(' ').collect();
            match splitted_instructions[0] {
                "addx" => {
                    register_a = splitted_instructions[1].parse::<i32>().unwrap();
                }
                "noop" => {}
                val => {
                    println!("Unrecognised instruction: {}", val);
                }
            }
            instruction_counter += 1;
        }
    }
    println!("Answer = {}", total);
}
pub fn run_2(input: &Vec<String>) {
    let mut register_x: i32 = 1;
    let mut register_a: i32 = 0;
    //let mut clock = 0;
    let mut instruction_counter = 0;
    let mut crt_pos: i32 = 0;
    let mut crt_line: String = String::new();
    while instruction_counter < input.len() {
        //clock += 1;
        if register_x -1 <= crt_pos && register_x + 1 >= crt_pos{
            crt_line.push('#');
        }else{
            crt_line.push('.');
        }
        crt_pos += 1;
        if crt_pos % 40 == 0{
            crt_line.push('\n');
            crt_pos = 0;
        }


        if register_a != 0 {
            register_x += register_a;
            register_a = 0;
        } else {
            let splitted_instructions: Vec<&str> = input[instruction_counter].split(' ').collect();
            match splitted_instructions[0] {
                "addx" => {
                    register_a = splitted_instructions[1].parse::<i32>().unwrap();
                }
                "noop" => {}
                val => {
                    println!("Unrecognised instruction: {}", val);
                }
            }
            instruction_counter += 1;
        }
    }
    println!("Answer = \n{}", crt_line);
}

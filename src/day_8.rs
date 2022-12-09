pub fn run_1(input: &Vec<String>) {
    let columns = input.len() as i32;
    let rows = input.get(0).unwrap().len() as i32;
    let mut matrix: Vec<Vec<usize>> = vec![];
    for tree_row in input {
        let mut temp_row: Vec<usize> = vec![];
        for char in tree_row.chars() {
            temp_row.push(char as usize);
        }
        matrix.push(temp_row);
    }
    let mut counter: i32 = rows + rows + columns + columns - 4;
    for y in 1..rows - 1 {
        for x in 1..columns - 1 {
            let current = matrix[y as usize][x as usize];
            let mut success: bool = true;
            while success == true {
                //left
                for x_neg in 0..x {
                    if matrix[y as usize][x_neg as usize] >= current {
                        success = false;
                        break;
                    }
                }
                if success == true {
                    break;
                }
                success = true;
                //right
                for x_pos in x + 1..rows {
                    if matrix[y as usize][x_pos as usize] >= current {
                        success = false;
                        break;
                    }
                }
                if success == true {
                    break;
                }
                success = true;
                //up
                for y_neg in 0..y {
                    if matrix[y_neg as usize][x as usize] >= current {
                        success = false;
                        break;
                    }
                }
                if success == true {
                    break;
                }
                success = true;
                //down
                for y_pos in y + 1..rows {
                    if matrix[y_pos as usize][x as usize] >= current {
                        success = false;
                        break;
                    }
                }
                if success == true {
                    break;
                }
            }
            if success == true {
                counter = counter + 1;
            }
        }
    }
    println!("Answer = {}", counter);
}

pub fn run_2(input: &Vec<String>) {
    //let all_trees: Vec<String> = utils::read_file_lines("./data/day_8_1.txt");
    let columns = input.len() as i32;
    let rows = input.get(0).unwrap().len() as i32;
    let mut matrix: Vec<Vec<usize>> = vec![];
    for tree_row in input {
        let mut temp_row: Vec<usize> = vec![];
        for char in tree_row.chars() {
            temp_row.push(char as usize);
        }
        matrix.push(temp_row);
    }
    let mut max_score = 0;
    let mut counter: usize;

    for y in 1..rows - 1 {
        for x in 1..columns - 1 {
            let current = matrix[y as usize][x as usize];
            let mut score: usize = 1;
            //left
            counter = 0;
            for x_neg in (0..x).rev() {
                counter += 1;
                if matrix[y as usize][x_neg as usize] >= current {
                    break;
                }
            }
            score *= counter;
            //right
            counter = 0;
            for x_pos in x + 1..rows {
                counter += 1;
                if matrix[y as usize][x_pos as usize] >= current {
                    break;
                }
            }
            score *= counter;
            //up
            counter = 0;
            for y_neg in (0..y).rev() {
                counter += 1;
                if matrix[y_neg as usize][x as usize] >= current {
                    break;
                }
            }
            score *= counter;
            //down
            counter = 0;
            for y_pos in y + 1..rows {
                counter += 1;
                if matrix[y_pos as usize][x as usize] >= current {
                    break;
                }
            }
            score *= counter;
            //tally
            if score > max_score {
                max_score = score.clone();
            }
        }
    }
    println!("Answer = {}", max_score);
}

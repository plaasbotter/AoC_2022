pub fn run_1(input: &Vec<String>) {
    let mut sand_start: Coord = Coord::new();
    let mut grid = extract_grid(input);
    for x in 0..grid[0].len() {
        if grid[0][x] == true {
            sand_start = Coord { x: x, y: 0 };
        }
    }
    let mut counter: usize = 0;
    'outer: loop {
        let mut sand_cur: Coord = sand_start.clone();
        'inner: loop {
            if sand_cur.y == grid.len() - 1 {
                break 'outer;
            } else if grid[sand_cur.y + 1][sand_cur.x] == false {
                sand_cur.y += 1;
            } else if grid[sand_cur.y + 1][sand_cur.x - 1] == false {
                sand_cur.y += 1;
                sand_cur.x -= 1;
            } else if grid[sand_cur.y + 1][sand_cur.x + 1] == false {
                sand_cur.y += 1;
                sand_cur.x += 1;
            } else {
                grid[sand_cur.y][sand_cur.x] = true;

                break 'inner;
            }
        }
        counter += 1;
    }
    println!("Answer = {}", counter);
}

fn _print_grid(grid: &Vec<Vec<bool>>) {
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

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new() -> Coord {
        return Coord { x: 0, y: 0 };
    }
}

fn extract_grid(input: &Vec<String>) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = vec![];
    let mut instructions: Vec<Vec<Coord>> = vec![];
    let mut smallest_x: usize = usize::MAX;
    let mut largest_x: usize = 0;
    let mut smallest_y: usize = usize::MAX;
    let mut largest_y: usize = 0;
    for line in input {
        let splitted_1: Vec<&str> = line.split(" -> ").collect();
        let mut temp_instructions: Vec<Coord> = vec![];
        for split_1 in splitted_1 {
            let splitted_2: Vec<&str> = split_1.split(',').collect();
            let x: usize = splitted_2[0].parse::<usize>().unwrap();
            let y: usize = splitted_2[1].parse::<usize>().unwrap();
            temp_instructions.push(Coord { x: x, y: y });
            if x > largest_x {
                largest_x = x;
            }
            if x < smallest_x {
                smallest_x = x;
            }
            if y > largest_y {
                largest_y = y;
            }
            if y < smallest_y {
                smallest_y = y;
            }
        }
        instructions.push(temp_instructions);
    }
    for _y in 0..=largest_y + 1 {
        let mut temp_row: Vec<bool> = vec![];
        for _x in smallest_x - 1..=largest_x + 1 {
            temp_row.push(false);
        }
        grid.push(temp_row);
    }
    for instruction in instructions {
        for i in 0..instruction.len() - 1 {
            let mut x_start = instruction[i].x - smallest_x + 1;
            let mut x_stop = instruction[i + 1].x - smallest_x + 1;
            if instruction[i + 1].x < instruction[i].x {
                x_start = instruction[i + 1].x - smallest_x + 1;
                x_stop = instruction[i].x - smallest_x + 1;
            }
            let mut y_start = instruction[i].y;
            let mut y_stop = instruction[i + 1].y;
            if instruction[i + 1].y < instruction[i].y {
                y_start = instruction[i + 1].y;
                y_stop = instruction[i].y;
            }

            for y in y_start..=y_stop {
                for x in x_start..=x_stop {
                    grid[y][x] = true;
                }
            }
        }
    }
    grid[0][500 - smallest_x + 1] = true; //add to top of grid to find later
    return grid;
}

fn extract_grid_2(input: &Vec<String>) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = vec![];
    let mut instructions: Vec<Vec<Coord>> = vec![];
    let mut smallest_x_coord: Coord = Coord { x: usize::MAX, y: 0 };
    let mut largest_x_coord: Coord = Coord { x: 0, y: 0 };
    let mut smallest_y: usize = usize::MAX;
    let mut largest_y: usize = 0;
    for line in input {
        let splitted_1: Vec<&str> = line.split(" -> ").collect();
        let mut temp_instructions: Vec<Coord> = vec![];
        for split_1 in splitted_1 {
            let splitted_2: Vec<&str> = split_1.split(',').collect();
            let x: usize = splitted_2[0].parse::<usize>().unwrap();
            let y: usize = splitted_2[1].parse::<usize>().unwrap();
            temp_instructions.push(Coord { x: x, y: y });
            if x > largest_x_coord.x {
                largest_x_coord.x = x;
                largest_x_coord.y = y;
            }
            if x < smallest_x_coord.x {
                smallest_x_coord.x = x;
                smallest_x_coord.y = y;
            }
            if y > largest_y {
                largest_y = y;
            }
            if y < smallest_y {
                smallest_y = y;
            }
        }
        instructions.push(temp_instructions);
    }

    largest_y += 3;
    let mut smallest_x = smallest_x_coord.x - (largest_y - smallest_x_coord.y)-2;
    smallest_x /= 2; //because i'm tired of thinking and the computer has the memory
    let mut largest_x = largest_x_coord.x + (largest_y - largest_x_coord.y)+2;
    largest_x *= 2; //because i'm tired of thinking and the computer has the memory
    for _y in 0..=largest_y {
        let mut temp_row: Vec<bool> = vec![];
        for _x in smallest_x - 1..=largest_x + 1 {
            temp_row.push(false);
        }
        grid.push(temp_row);
    }
    for instruction in instructions {
        for i in 0..instruction.len() - 1 {
            let mut x_start = instruction[i].x - smallest_x + 1;
            let mut x_stop = instruction[i + 1].x - smallest_x + 1;
            if instruction[i + 1].x < instruction[i].x {
                x_start = instruction[i + 1].x - smallest_x + 1;
                x_stop = instruction[i].x - smallest_x + 1;
            }
            let mut y_start = instruction[i].y;
            let mut y_stop = instruction[i + 1].y;
            if instruction[i + 1].y < instruction[i].y {
                y_start = instruction[i + 1].y;
                y_stop = instruction[i].y;
            }

            for y in y_start..=y_stop {
                for x in x_start..=x_stop {
                    grid[y][x] = true;
                }
            }
        }
    }
    for x in 0..grid[0].len(){
        grid[largest_y-1][x] = true;
    }
    grid[0][500 - smallest_x + 1] = true; //add to top of grid to find later
    return grid;
}

pub fn run_2(input: &Vec<String>) {
    let mut sand_start: Coord = Coord::new();
    let mut grid = extract_grid_2(input);
    for x in 0..grid[0].len() {
        if grid[0][x] == true {
            sand_start = Coord { x: x, y: 0 };
        }
    }
    let mut counter: usize = 0;
    'outer: loop {
        counter += 1;
        let mut sand_cur: Coord = sand_start.clone();
        'inner: loop {
            if grid[sand_cur.y + 1][sand_cur.x] == false {
                sand_cur.y += 1;
            } else if grid[sand_cur.y + 1][sand_cur.x - 1] == false {
                sand_cur.y += 1;
                sand_cur.x -= 1;
            } else if grid[sand_cur.y + 1][sand_cur.x + 1] == false {
                sand_cur.y += 1;
                sand_cur.x += 1;
            }
            else if sand_cur.y == sand_start.y && sand_cur.x == sand_start.x{
                break 'outer
            } else {
                grid[sand_cur.y][sand_cur.x] = true;

                break 'inner;
            }
        }
    }
    //print_grid(&grid);
    println!("Answer = {}", counter);
}

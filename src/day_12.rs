use std::{collections::HashSet, vec};

#[derive(Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
    steps: usize,
    distance: usize,
    score: usize,
    val: char,
    path: String,
}

impl Coord {
    fn get_string(&self) -> String {
        return format!("{},{}", self.x, self.y);
    }
}

fn load_grid(start: &mut Coord, end: &mut Coord, input: &Vec<String>) -> Vec<Vec<i16>> {
    let mut row = 0;
    let mut column;
    let mut grid: Vec<Vec<i16>> = vec![];
    for line in input {
        let mut temp_line: Vec<i16> = vec![];
        let byte_arr = line.as_bytes();
        column = 0;
        for byte in byte_arr {
            match byte {
                b'S' => {
                    temp_line.push(97);
                    start.x = column;
                    start.y = row
                }
                b'E' => {
                    temp_line.push(122);
                    end.x = column;
                    end.y = row
                }
                val => temp_line.push(*val as i16),
            }
            column += 1;
        }
        grid.push(temp_line);
        row += 1;
    }
    return grid;
}

pub fn run_1(input: &Vec<String>) {
    let rows: usize = input.len();
    let columns: usize = input[0].len();
    let mut start: Coord = Coord {
        x: 0,
        y: 0,
        steps: 0,
        distance: 0,
        score: 0,
        val: 'Q',
        path: String::new(),
    };
    let mut end: Coord = start.clone();
    let grid: Vec<Vec<i16>> = load_grid(&mut start, &mut end, input);
    let steps = solve(&mut start, &end, &grid, rows, columns);
    //_print_grid2(&rows, &columns, &grid, cur_coord.path.clone());
    println!("Answer = {}", steps);
}

fn add_locations(
    grid: &Vec<Vec<i16>>,
    mut temp_cord: Coord,
    cur_coord: &Coord,
    end: &Coord,
    possible_locations: &mut Vec<Coord>,
) {
    if (grid[temp_cord.y][temp_cord.x] - grid[cur_coord.y][cur_coord.x]) <= 1 {
        temp_cord.distance = get_distance(&end, &temp_cord);
        temp_cord.steps += 1;
        temp_cord.score = (temp_cord.steps * temp_cord.steps) + temp_cord.distance;
        temp_cord.val = grid[temp_cord.y][temp_cord.x] as u8 as char;
        possible_locations.push(temp_cord);
    }
}

fn get_distance(end: &Coord, start: &Coord) -> usize {
    let distance = ((end.y as i32 - start.y as i32) * (end.y as i32 - start.y as i32))
        + ((end.x as i32 - start.x as i32) * (end.x as i32 - start.x as i32));
    return (distance as f32).sqrt() as usize;
}

fn _print_grid(rows: &usize, columns: &usize, previous_locations: &HashSet<String>) {
    let mut grid: Vec<Vec<String>> = vec![];
    for _y in 0..*rows {
        let mut temp_row: Vec<String> = vec![];
        for _x in 0..*columns {
            temp_row.push(String::from("."));
        }
        grid.push(temp_row);
    }
    for location in previous_locations {
        let splitted: Vec<&str> = location.split(',').collect();
        let x = splitted[0].parse::<usize>().unwrap();
        let y = splitted[1].parse::<usize>().unwrap();
        grid[y][x] = String::from("#");
    }
    for row in grid {
        println!("{:?}", row);
    }
}

fn _print_grid2(rows: &usize, columns: &usize, in_grid: &Vec<Vec<i16>>, path: String) {
    let mut grid: Vec<Vec<char>> = vec![];
    for y in 0..*rows {
        let mut temp_row: Vec<char> = vec![];
        for x in 0..*columns {
            temp_row.push(in_grid[y][x] as u8 as char);
        }
        grid.push(temp_row);
    }
    let splitted_1: Vec<&str> = path.split('|').collect();
    for split in splitted_1 {
        if split.len() > 0 {
            let splitted_2: Vec<&str> = split.split(',').collect();
            let x = splitted_2[0].parse::<usize>().unwrap();
            let y = splitted_2[1].parse::<usize>().unwrap();
            //grid[y][x] = (grid[y][x] as u8 - 32) as char;
            grid[y][x] = '#';
        }
    }
    for row in grid {
        println!("{:?}", row);
    }
}

pub fn run_2(input: &Vec<String>) {
    let mut smallest: usize = usize::MAX;
    let rows: usize = input.len();
    let columns: usize = input[0].len();
    let mut start: Coord = Coord {
        x: 0,
        y: 0,
        steps: 0,
        distance: 0,
        score: 0,
        val: 'Q',
        path: String::new(),
    };
    let mut end: Coord = start.clone();
    let grid: Vec<Vec<i16>> = load_grid(&mut start, &mut end, input);

    for y in 0..grid.len() {
        for x in 0..1 {
            if grid[y][x] == 97 {
                start = Coord {
                    x: x,
                    y: y,
                    steps: 0,
                    distance: 0,
                    score: 0,
                    val: 'Q',
                    path: String::new(),
                };
                let steps = solve(&mut start, &end, &grid, rows, columns);
                if steps < smallest {
                    smallest = steps;
                }
            }
        }
    }
    println!("Answer = {}", smallest);
}

fn solve(
    start: &mut Coord,
    end: &Coord,
    grid: &Vec<Vec<i16>>,
    rows: usize,
    columns: usize,
) -> usize {
    let mut save_scum: Vec<Coord> = vec![];
    start.distance = get_distance(end, &*start);
    start.score = start.steps + start.distance;
    start.val = grid[start.y][start.x] as u8 as char;
    let mut cur_coord: Coord = start.clone();
    save_scum.push(start.clone());
    let mut previous_locations: HashSet<String> = HashSet::new();
    let mut possible_locations: Vec<Coord>;
    loop {
        if cur_coord.x == end.x && cur_coord.y == end.y {
            break;
        }
        cur_coord = save_scum.pop().unwrap();

        possible_locations = Vec::new();
        let mut temp_cord: Coord;

        //DOWN
        temp_cord = cur_coord.clone();
        temp_cord.y += 1;
        if temp_cord.y < rows {
            add_locations(grid, temp_cord, &cur_coord, end, &mut possible_locations);
        }

        //UP
        temp_cord = cur_coord.clone();
        if temp_cord.y >= 1 {
            temp_cord.y -= 1;
            add_locations(grid, temp_cord, &cur_coord, end, &mut possible_locations);
        }

        //RIGHT
        temp_cord = cur_coord.clone();
        temp_cord.x += 1;
        if temp_cord.x < columns {
            add_locations(grid, temp_cord, &cur_coord, end, &mut possible_locations);
        }

        //LEFT
        temp_cord = cur_coord.clone();
        if temp_cord.x >= 1 {
            temp_cord.x -= 1;
            add_locations(grid, temp_cord, &cur_coord, end, &mut possible_locations);
        }

        while possible_locations.len() > 0 {
            let mut local: Coord = possible_locations.pop().unwrap();
            let name = &local.get_string();
            if previous_locations.contains(name) == true {
                'inner: for save in &mut save_scum {
                    if local.x == save.x && local.y == save.y {
                        if local.steps < save.steps {
                            save.path = local.path.clone();
                            save.steps = local.steps.clone();
                        }
                        break 'inner;
                    }
                }
            } else {
                local
                    .path
                    .push_str(format!("|{},{}", cur_coord.x, cur_coord.y).as_str());
                save_scum.push(local);
                previous_locations.insert(name.clone());
            }
        }

        save_scum.sort_by(|a, b| b.score.cmp(&a.score));
        if save_scum.len() == 0 {
            break;
        }
    }
    //_print_grid2(&rows, &columns, grid, cur_coord.path);
    return cur_coord.steps;
}

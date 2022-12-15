use crate::utils::{_check_size};

#[derive(Debug, Clone)]
struct Coord {
    x: isize,
    y: isize,
    d: isize,
}

pub fn run_1(input: &Vec<String>) {
    let mut largest_x: isize = isize::MIN;
    let mut smallest_x: isize = isize::MAX;
    let mut sensors: Vec<Coord> = vec![];
    let mut beacons: Vec<Coord> = vec![];
    load_sensors(
        input,
        &mut largest_x,
        &mut smallest_x,
        &mut beacons,
        &mut sensors,
    );

    let mut grid: Vec<u8> = Vec::new();
    for _i in smallest_x..=largest_x {
        grid.push(0);
    }
    let shift_x: isize = 0 - smallest_x;
    let mut line_to_check = 2000000;
    if largest_x < 100 {
        line_to_check = 10;
    }
    let mut counter: usize = 0;
    for beacon in beacons {
        if beacon.y == line_to_check {
            grid[beacon.x as usize + shift_x as usize] = 2;
        }
    }
    for x in 0..grid.len() {
        'inner: for sensor in &sensors {
            let dist = (line_to_check - sensor.y).abs() + (x as isize - shift_x - sensor.x).abs();
            if dist <= sensor.d {
                if grid[x] == 0 {
                    grid[x] = 1;
                    counter += 1;
                    break 'inner;
                }
            }
        }
    }
    println!("Answer = {}", counter);
}

fn load_sensors(
    input: &Vec<String>,
    largest_x: &mut isize,
    smallest_x: &mut isize,
    beacons: &mut Vec<Coord>,
    sensors: &mut Vec<Coord>,
) {
    for line in input {
        let mut newline = line.clone();
        newline = newline.replace("=", " ");
        newline = newline.replace(":", " ");
        newline = newline.replace(",", " ");
        let splitted: Vec<&str> = newline.split(' ').collect();
        let temp_x_s = splitted[3].parse::<isize>().unwrap();
        let temp_y_s = splitted[6].parse::<isize>().unwrap();
        let mut temp_sensor = Coord {
            x: temp_x_s,
            y: temp_y_s,
            d: 0,
        };
        let temp_x_b = splitted[13].parse::<isize>().unwrap();
        let temp_y_b = splitted[16].parse::<isize>().unwrap();
        temp_sensor.d = (temp_x_b - temp_sensor.x).abs() + (temp_y_b - temp_sensor.y).abs();
        _check_size(temp_x_s + temp_sensor.d, largest_x, smallest_x);
        _check_size(temp_x_s - temp_sensor.d, largest_x, smallest_x);
        _check_size(temp_x_b, largest_x, smallest_x);
        beacons.push(Coord {
            x: temp_x_b,
            y: temp_y_b,
            d: 0,
        });
        sensors.push(temp_sensor.clone());
    }
}

pub fn run_2(input: &Vec<String>) {
    let mut largest_x: isize = isize::MIN;
    let mut smallest_x: isize = isize::MAX;
    let mut sensors: Vec<Coord> = vec![];
    let mut beacons: Vec<Coord> = vec![];
    load_sensors(
        input,
        &mut largest_x,
        &mut smallest_x,
        &mut beacons,
        &mut sensors,
    );
    let mut final_coord: Coord = Coord { x: 0, y: 0, d: 0 };
    let mut max: isize = 4000000;
    if largest_x < 100 {
        max = 20
    }
    let mut found: bool = false;
    'outer: for sensor_outer in &sensors {
        let new_d = sensor_outer.d + 1;
        for x in sensor_outer.x - new_d..sensor_outer.x + new_d {
            if x >= 0 && x <= max {
                let y_dist = new_d - (x - sensor_outer.x).abs();
                let d_up = sensor_outer.y - y_dist;
                let d_down = sensor_outer.y + y_dist;
                if d_up >= 0 && d_up <= max {
                    search_sensors(&mut found, &sensors, d_up, x);
                    if found == true {
                        final_coord = Coord {
                            x: x,
                            y: d_up,
                            d: 0,
                        };
                        break 'outer;
                    }
                }
                if d_down >= 0 && d_down <= max {
                    search_sensors(&mut found, &sensors, d_down, x);
                    if found == true {
                        final_coord = Coord {
                            x: x,
                            y: d_down,
                            d: 0,
                        };
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("Answer = {}", (4000000 * final_coord.x) + final_coord.y);
}

fn search_sensors(found: &mut bool, sensors: &Vec<Coord>, y: isize, x: isize) {
    *found = true;
    'inner: for sensor_inner in sensors {
        let dist =
            (y - sensor_inner.y).abs() + (x as isize - sensor_inner.x).abs();
        if dist <= sensor_inner.d {
            *found = false;
            break 'inner;
        }
    }
}

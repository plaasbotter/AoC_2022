use std::collections::HashSet;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new() -> Coord {
        return Coord { x: 0, y: 0 };
    }
}

pub fn run_1(input: &Vec<String>) {
    let mut map: HashSet<String> = HashSet::new();
    let mut h: Coord = Coord::new();
    let mut h_new: Coord = Coord::new();
    let mut t: Coord = Coord::new();
    let mut x_addition: i32;
    let mut y_addition: i32;
    map.insert(String::from("0,0"));
    for movement in input {
        let splitted: Vec<&str> = movement.split(' ').collect();
        let direction = splitted[0];
        let distance = splitted[1].parse::<i32>().unwrap();
        x_addition = 0;
        y_addition = 0;
        match direction {
            "U" => {
                h_new.y += distance;
                y_addition = 1;
            }
            "D" => {
                h_new.y -= distance;
                y_addition = -1;
            }
            "L" => {
                h_new.x -= distance;
                x_addition = -1;
            }
            "R" => {
                h_new.x += distance;
                x_addition = 1;
            }
            val => {
                println!("Uknown command: {}", val);
            }
        }
        while h_new.x != h.x || h_new.y != h.y {
            h.x += x_addition;
            h.y += y_addition;
            let x_dist = t.x - h.x;
            let y_dist = t.y - h.y;
            if x_dist.abs() >= 2 || y_dist.abs() >= 2 {
                t.y += y_addition;
                if x_addition != 0 && y_dist != 0 {
                    t.y = h.y;
                }
                t.x += x_addition;
                if y_addition != 0 && x_dist != 0 {
                    t.x = h.x;
                }
            }
            map.insert(format!("{},{}", t.x, t.y));
        }
    }
    println!("Answer = {}", map.len());
}

pub fn run_2(input: &Vec<String>) {
    let mut map: HashSet<String> = HashSet::new();
    let mut rope: Vec<Coord> = vec![];
    let mut h_new: Coord = Coord::new();
    for _i in 0..10 {
        rope.push(Coord::new());
    }
    let mut x_addition: i32;
    let mut y_addition: i32;
    map.insert(String::from("0,0"));
    let mut direction: &str;
    let mut distance: i32;
    for movement in input {
        let splitted: Vec<&str> = movement.split(' ').collect();
        direction = splitted[0];
        distance = splitted[1].parse::<i32>().unwrap();
        x_addition = 0;
        y_addition = 0;
        match direction {
            "U" => {
                h_new.y += distance;
                y_addition = 1;
            }
            "D" => {
                h_new.y -= distance;
                y_addition = -1;
            }
            "L" => {
                h_new.x -= distance;
                x_addition = -1;
            }
            "R" => {
                h_new.x += distance;
                x_addition = 1;
            }
            val => {
                println!("Uknown command: {}", val);
            }
        }
        while h_new.x != rope[0].x || h_new.y != rope[0].y {
            rope[0].x += x_addition;
            rope[0].y += y_addition;
            let mut x_dist: i32;
            let mut y_dist: i32;
            for i in 1..rope.len(){
                x_dist = rope[i].x - rope[i-1].x;
                y_dist = rope[i].y - rope[i-1].y;
                if x_dist.abs() >= 2 || y_dist.abs() >= 2 {
                    if x_dist == 1 || x_dist == -1{
                        rope[i].x = rope[i-1].x;
                    }else{
                        rope[i].x = (rope[i-1].x + rope[i].x)/2;
                    }
                    if y_dist == 1 || y_dist == -1{
                        rope[i].y = rope[i-1].y;
                    }else{
                        rope[i].y = (rope[i-1].y + rope[i].y)/2;
                    }
                }
            }
            map.insert(format!("{},{}",rope[rope.len()-1].x,rope[rope.len()-1].y));
        }
    }
    println!("Answer = {}", map.len());
}

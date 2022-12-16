use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Valve {
    id: String,
    rate: usize,
    close: bool,
    tunnel: Vec<String>,
}

impl Valve {
    fn new() -> Valve {
        return Valve {
            id: String::new(),
            rate: 0,
            close: false,
            tunnel: vec![],
        };
    }
}

pub fn run_1(input: &Vec<String>) {
    let mut all_valves: HashMap<String, Valve> = HashMap::new();
    load_valves(input, &mut all_valves);
    let time: usize = 29;
    let pressure: usize = 0;
    let mut max: usize = 0;
    let cur_valve = all_valves["AA"].clone();
    repo(time, &mut max, pressure, cur_valve, all_valves);
    //repo2(time, &mut max, cur_valve, all_valves);
    println!("Answer = {}", max);
}

fn repo2(time: usize, max: &mut usize, cur_valve: Valve, mut all_valves: HashMap<String, Valve>) {
    println!("repo_2_start");
    let mut possibles: Vec<String> = cur_valve.tunnel.clone();
    let mut max_possible_value: usize = 0;
    let mut max_steps: usize = 0;
    let mut new_cur_valve: Valve = Valve::new();
    let mut steps: usize = 1;
    let mut storage_max: usize = 0;
    loop {
        if time < (steps + 1) {
            return;
        }
        for possible in &possibles {
            if all_valves[possible].rate > 0 {
                let temp_valve = &all_valves[possible];
                let value = temp_valve.rate * (time - steps - 1);
                if value > max_possible_value {
                    max_possible_value = value;
                    new_cur_valve = temp_valve.clone();
                    max_steps = steps;
                }
            }
        }
        println!("{} {}", max_possible_value, storage_max);
        if storage_max != 0 && max_possible_value <= storage_max {
            break;
        }
        storage_max = max_possible_value;
        steps += 1;
        let mut new_possibles: Vec<String> = vec![];
        for possible in &possibles {
            for tuns in &all_valves[possible].tunnel {
                new_possibles.push(tuns.clone());
            }
        }
        possibles = new_possibles.clone();
    }
    all_valves.get_mut(&new_cur_valve.id).unwrap().rate = 0;
    *max += max_possible_value;
    println!("m{}m{}", max, max_possible_value);
    repo2(time - max_steps - 1, max, new_cur_valve, all_valves);
}

fn repo(
    time: usize,
    max: &mut usize,
    pressure: usize,
    cur_valve: Valve,
    mut all_valves: HashMap<String, Valve>,
) {
    let mut new_time = time.clone() - 1;
    let mut new_pressure = pressure.clone();
    if new_time > 0 && cur_valve.rate > 0 {
        new_time -= 1;
        new_pressure += new_time * cur_valve.rate;
        all_valves.get_mut(&cur_valve.id).unwrap().rate = 0;
    }
    if new_time == 0 {
        //println!("Final Move");
        //println!("new_pre={}", new_pressure);
        //println!("max={}", max);
        if new_pressure > *max {
            *max = new_pressure;
            println!("max={}", max);
        }
    } else {
        let new_all_valves = all_valves.clone();
        let mut list_of_possibles: Vec<Valve> = vec![];
        for valve in cur_valve.tunnel {
            let temp_valve = new_all_valves.get(&valve).unwrap().clone();
            list_of_possibles.push(temp_valve.clone());
            list_of_possibles.sort_by(|a, b| b.rate.cmp(&a.rate))
        }
        for possible in list_of_possibles {
            repo(
                new_time.clone(),
                max,
                new_pressure.clone(),
                possible.clone(),
                new_all_valves.clone(),
            );
        }
    }
}

fn load_valves(input: &Vec<String>, all_valves: &mut HashMap<String, Valve>) {
    for line in input {
        let mut new_line: String = line.clone();
        new_line = new_line.replace("=", " ").replace(";", "").replace(",", "");
        let splitted: Vec<&str> = new_line.split(' ').collect();
        let mut temp_valve: Valve = Valve::new();
        let id: String = splitted[1].to_string();
        temp_valve.id = id.clone();
        temp_valve.rate = splitted[5].parse::<usize>().unwrap();
        temp_valve.close = false;
        for i in 10..splitted.len() {
            temp_valve.tunnel.push(splitted[i].to_string());
        }
        all_valves.insert(id, temp_valve);
    }
}

pub fn run_2(input: &Vec<String>) {
    println!("Answer = {}", 0);
}

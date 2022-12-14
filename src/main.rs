mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
//mod day_14;
//mod day_15;
//mod day_16;
//mod day_17;
//mod day_18;
//mod day_19;
//mod day_20;
//mod day_21;
//mod day_22;
//mod day_23;
//mod day_24;
//mod day_25;
mod utils;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        println!("Day1");
        let day_1_data: Vec<String> = utils::read_file_lines("./data/day_1_1.txt");
        day_1::run_1(&day_1_data);
        day_1::run_2(&day_1_data);
        //
        println!("Day2");
        let day_2_data: Vec<String> = utils::read_file_lines("./data/day_2_1.txt");
        day_2::run_1(&day_2_data);
        day_2::run_2(&day_2_data);
        //
        println!("Day3");
        let day_3_data: Vec<String> = utils::read_file_lines("./data/day_3_1.txt");
        day_3::run_1(&day_3_data);
        day_3::run_2(&day_3_data);
        //
        println!("Day4");
        let day_4_data: Vec<String> = utils::read_file_lines("./data/day_4_1.txt");
        day_4::run_1(&day_4_data);
        day_4::run_2(&day_4_data);
        //
        println!("Day5");
        let day_5_data: Vec<String> = utils::read_file_lines("./data/day_5_1.txt");
        day_5::run_1(&day_5_data);
        day_5::run_2(&day_5_data);
        //
        println!("Day6");
        let day_6_data: Vec<String> = utils::read_file_lines("./data/day_6_1.txt");
        day_6::run_1(&day_6_data);
        day_6::run_2(&day_6_data);
        //
        println!("Day7");
        let day_7_data: Vec<String> = utils::read_file_lines("./data/day_7_1.txt");
        day_7::run_1(&day_7_data);
        day_7::run_2(&day_7_data);
        //
        println!("Day8");
        let day_8_data: Vec<String> = utils::read_file_lines("./data/day_8_1.txt");
        day_8::run_1(&day_8_data);
        day_8::run_2(&day_8_data);
        //
        println!("Day9");
        let day_9_data: Vec<String> = utils::read_file_lines("./data/day_9_1.txt");
        day_9::run_1(&day_9_data);
        day_9::run_2(&day_9_data);
        //
        println!("Day10");
        let day_10_data: Vec<String> = utils::read_file_lines("./data/day_10_1.txt");
        day_10::run_1(&day_10_data);
        day_10::run_2(&day_10_data);
        //
        println!("Day11");
        let day_11_data: Vec<String> = utils::read_file_lines("./data/day_11_1.txt");
        day_11::run_1(&day_11_data);
        day_11::run_2(&day_11_data);
        //
        println!("Day12");
        let day_12_data: Vec<String> = utils::read_file_lines("./data/day_12_1.txt");
        day_12::run_1(&day_12_data);
        day_12::run_2(&day_12_data);
        //
        println!("Day13");
        let day_13_data: Vec<String> = utils::read_file_lines("./data/day_13_1.txt");
        day_13::run_1(&day_13_data);
        day_13::run_2(&day_13_data);
        //
        //println!("Day14");
        //let day_14_data: Vec<String> = utils::read_file_lines("./data/day_14_1.txt");
        //day_14::run_1(&day_14_data);
        //day_14::run_2(&day_14_data);
        ////
        //println!("Day15");
        //let day_15_data: Vec<String> = utils::read_file_lines("./data/day_15_1.txt");
        //day_15::run_1(&day_15_data);
        //day_15::run_2(&day_15_data);
        ////
        //println!("Day16");
        //let day_16_data: Vec<String> = utils::read_file_lines("./data/day_16_1.txt");
        //day_16::run_1(&day_16_data);
        //day_16::run_2(&day_16_data);
        ////
        //println!("Day17");
        //let day_17_data: Vec<String> = utils::read_file_lines("./data/day_17_1.txt");
        //day_17::run_1(&day_17_data);
        //day_17::run_2(&day_17_data);
        ////
        //println!("Day18");
        //let day_18_data: Vec<String> = utils::read_file_lines("./data/day_18_1.txt");
        //day_18::run_1(&day_18_data);
        //day_18::run_2(&day_18_data);
        ////
        //println!("Day19");
        //let day_19_data: Vec<String> = utils::read_file_lines("./data/day_19_1.txt");
        //day_19::run_1(&day_19_data);
        //day_19::run_2(&day_19_data);
        ////
        //println!("Day20");
        //let day_20_data: Vec<String> = utils::read_file_lines("./data/day_20_1.txt");
        //day_20::run_1(&day_20_data);
        //day_20::run_2(&day_20_data);
        ////
        //println!("Day21");
        //let day_21_data: Vec<String> = utils::read_file_lines("./data/day_21_1.txt");
        //day_21::run_1(&day_21_data);
        //day_21::run_2(&day_21_data);
        ////
        //println!("Day22");
        //let day_22_data: Vec<String> = utils::read_file_lines("./data/day_22_1.txt");
        //day_22::run_1(&day_22_data);
        //day_22::run_2(&day_22_data);
        ////
        //println!("Day23");
        //let day_23_data: Vec<String> = utils::read_file_lines("./data/day_23_1.txt");
        //day_23::run_1(&day_23_data);
        //day_23::run_2(&day_23_data);
        ////
        //println!("Day24");
        //let day_24_data: Vec<String> = utils::read_file_lines("./data/day_24_1.txt");
        //day_24::run_1(&day_24_data);
        //day_24::run_2(&day_24_data);
        ////
        //println!("Day25");
        //let day_25_data: Vec<String> = utils::read_file_lines("./data/day_25_1.txt");
        //day_25::run_1(&day_25_data);
        //day_25::run_2(&day_25_data);
        ////
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

mod utils;
mod day_1;
mod day_2;
mod day_3;
mod day_7;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        println!("Day1");
        day_1::run_1();
        day_1::run_2();
        println!("Day2");
        day_2::run_1();
        day_2::run_2();
        println!("Day3");
        day_3::run_1();
        day_3::run_2();
        println!("Day7");
        day_7::run_1();
        day_7::run_2();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
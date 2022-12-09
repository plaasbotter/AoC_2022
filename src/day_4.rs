pub fn run_1(input: &Vec<String>) {
    let mut counter: usize = 0;
    for job in input {
        let new_job = job.replace(",", "-");
        let new_job_list: Vec<&str> = new_job.split('-').collect();
        let a = new_job_list[0].parse::<usize>().unwrap();
        let b = new_job_list[1].parse::<usize>().unwrap();
        let c = new_job_list[2].parse::<usize>().unwrap();
        let d = new_job_list[3].parse::<usize>().unwrap();
        if (a <= c && b >= d) || (a >= c && b <= d) {
            counter += 1;
        }
    }
    println!("Answer is {}", counter);
}

pub fn run_2(input: &Vec<String>) {
    let mut counter: usize = 0;
    for job in input {
        let new_job = job.replace(",", "-");
        let new_job_list: Vec<&str> = new_job.split('-').collect();
        let a = new_job_list[0].parse::<usize>().unwrap();
        let b = new_job_list[1].parse::<usize>().unwrap();
        let c = new_job_list[2].parse::<usize>().unwrap();
        let d = new_job_list[3].parse::<usize>().unwrap();
        if (a >= c && a <= d) || (b >= c && b <= d) {
            counter += 1;
        } else if (c >= a && c <= b) || (d >= a && d <= b) {
            counter += 1;
        }
    }
    println!("Answer is {}", counter);
}

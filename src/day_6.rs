use std::collections::VecDeque;

pub fn run_1(input: &Vec<String>) {
    let mut charlist: VecDeque<u8> = VecDeque::new();
    let mut answ: usize = 0;
    let length: usize = 4;
    for line in input{
        let byte_representation = line.as_bytes();
        for i in 0..line.len(){
            charlist.push_back(byte_representation[i]);
            if charlist.len() >= length{
                if run_test(&charlist, &length){
                    answ = i + 1;
                    break;
                }
                charlist.pop_front();
            }
        }
    }
    println!("Answer = {}", answ);
}

fn run_test(charlist: &VecDeque<u8>, length: &usize) -> bool {
    for i in 0..*length{
        for j in i+1..*length{
            if charlist[i] == charlist[j]{
                return false;
            }
        }
    }
    return true;
}
pub fn run_2(input: &Vec<String>) {
    let mut charlist: VecDeque<u8> = VecDeque::new();
    let mut answ: usize = 0;
    let length: usize = 14;
    for line in input{
        let byte_representation = line.as_bytes();
        for i in 0..line.len(){
            charlist.push_back(byte_representation[i]);
            if charlist.len() >= length{
                if run_test(&charlist, &length){
                    answ = i + 1;
                    break;
                }
                charlist.pop_front();
            }
        }
    }
    println!("Answer = {}", answ);
}

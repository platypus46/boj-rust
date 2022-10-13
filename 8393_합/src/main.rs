use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input error");

    let nums: Vec<&str> = input.split_whitespace().collect();

    let num: i32 = nums[0].parse::<i32>().unwrap();
    let mut Sum=0;
    for i in 1..num+1{
        Sum+=i;
    }
    println!("{}",Sum);
}
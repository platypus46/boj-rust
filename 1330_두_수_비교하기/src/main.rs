use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input error");

    let nums: Vec<&str> = input.split_whitespace().collect();

    let num1: f64 = nums[0].parse::<f64>().unwrap();
    let num2: f64 = nums[1].parse::<f64>().unwrap();

    if num1 > num2 {
        println!(">");
    } else if num1 < num2 {
        println!("<");
    } else {
        println!("==");
    }
}
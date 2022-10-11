use std::io;

fn main() {
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("input reading error");
    let nums:Vec<usize>=input.split_whitespace().map(|s| s.trim().parse().expect("input parsing error")).collect::<Vec<_>>();

    println!("{}",nums[0]+nums[1])
}
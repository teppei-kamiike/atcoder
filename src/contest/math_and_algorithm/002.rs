use std::io::{self};

fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let inputs = input.trim().split(" ");
    let mut sum = 0;
    for input_str in inputs {
        let input_int: i32 = input_str.parse().unwrap();
        sum += input_int;
    }

    println!("{}", sum)
}

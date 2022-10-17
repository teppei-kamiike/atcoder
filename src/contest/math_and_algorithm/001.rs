use std::io::{self};

fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let input_int: i32 = input.trim().parse().unwrap();

    println!("{}", input_int + 5);
}

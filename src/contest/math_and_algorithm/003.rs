use std::io::{self};

fn main() {
    let mut size = String::new();
    let mut input_numbers = String::new();
    let _ = io::stdin().read_line(&mut size);
    let _ = io::stdin().read_line(&mut input_numbers);

    let numbers = input_numbers.trim().split(" ");
    let mut sum = 0;
    for number_str in numbers {
        let number_int: i32 = number_str.parse().unwrap();
        sum +=  number_int;
    }

    println!("{}", sum)
}

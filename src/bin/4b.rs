use advent_of_code_2019::{parser, password};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let range: Vec<&str> = args[1].split('-').map(|s|s.trim()).collect();
    let (min, max) = (range[0].parse::<i32>().unwrap(), range[1].parse::<i32>().unwrap());

    let mut counter = 0;
    let mut current = password::Password::new(min);
    while current.value() <= max {
        if current.is_strictly_valid() {
            counter += 1;
        }
        match current.next() {
            Some(p) => current = p,
            _ => break,
        }
    }

    println!("\"Strictly\" valid passwords within range ({} - {}): {}", min, max, counter);
}
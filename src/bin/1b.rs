extern crate advent_of_code_2019;

use advent_of_code_2019::{parser, sum};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args {
        print!("{} ", arg);
    }
    println!();

    let modules = parser::parse_lines(args);

    println!(
        "Fuel needed: {}",
        sum::sum_fn_parsed(modules.iter(), fuel_needed)
    );
}

fn fuel_needed(weight: i32) -> i32 {
    match weight / 3 - 2 {
        x if x < 1 => 0,
        x => x + fuel_needed(x),
    }
}

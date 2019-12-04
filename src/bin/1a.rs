extern crate advent_of_code_2019;

use std::env;
use advent_of_code_2019::{lines, sum};

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.clone() {
        println!("{}", arg);
    }

    let modules = lines::parse_lines(args);

    println!("Fuel needed: {}", sum::sum_fn_parsed(modules.iter(), fuel_needed));
}

fn fuel_needed(weight: i32) -> i32 {
    weight / 3 - 2
}

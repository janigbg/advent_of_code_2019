extern crate advent_of_code_2019;

use advent_of_code_2019::{intcode, parser};
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.clone() {
        println!("{}", arg);
    }

    let mut program = program(args);

    let mut pc = 0;
    while intcode::process_instruction(&mut program, &mut pc) {}

    println!("Result: {}", program[0]);
}

fn program(args: Vec<String>) -> Vec<i32> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

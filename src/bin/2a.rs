extern crate advent_of_code_2019;

use advent_of_code_2019::{intcode, parser};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let mut program = program(&args);

    let mut pc = 0;
    let mut rb = 0;
    while let Ok(true) = intcode::process_instruction(&mut program, &mut pc, &mut rb, &mut intcode::err_input, &mut intcode::stdout_output) {}

    println!("Result: {}", program[0]);
}

fn program(args: &Vec<String>) -> Vec<i64> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

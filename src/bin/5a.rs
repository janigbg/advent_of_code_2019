extern crate advent_of_code_2019;

use advent_of_code_2019::{intcode, parser};
use std::env;
use std::error::Error;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let mut program = program(&args);

    let mut pc = 0;
    while let Ok(true) = intcode::process_instruction(&mut program, &mut pc, stdin) {}

    println!("Done: {}", program[0]);
}

fn stdin() -> Result<i32, Box<dyn Error>> {
    print!("INPUT: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse::<i32>()?)
}

fn program(args: &Vec<String>) -> Vec<i32> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

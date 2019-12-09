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

    for _ in 0..10000 {
        program.push(0);
    }

    let mut pc = 0;
    let mut rb = 0;
    while let Ok(true) = intcode::process_instruction(&mut program, &mut pc, &mut rb, &mut stdin, &mut intcode::stdout_output) {}

    println!("Done: {}", program[0]);
}

fn stdin() -> Result<i64, Box<dyn Error>> {
    print!("INPUT: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse::<i64>()?)
}

fn program(args: &Vec<String>) -> Vec<i64> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

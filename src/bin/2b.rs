extern crate advent_of_code_2019;

use advent_of_code_2019::{intcode, parser};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let program = program(&args);

    let expected = 19690720;
    let mut found: Option<(i64, i64)> = None;

    for i in 0..10000 {
        let (noun, verb) = (i / 100, i % 100);
        if test_program(expected, &program, noun, verb) {
            found = Some((noun, verb));
            break;
        }
    }

    match found {
        Some((noun, verb)) => println!(
            "Result {} found for Noun: {}, Verb: {}",
            noun * 100 + verb,
            noun,
            verb
        ),
        None => println!("Result not found!"),
    };
}

fn test_program(expected: i64, program: &Vec<i64>, noun: i64, verb: i64) -> bool {
    println!("Testing Noun: {}, Verb: {}", noun, verb);
    let mut prog = program.clone();
    prog[1] = noun;
    prog[2] = verb;
    let mut pc = 0;
    let mut rb = 0;
    while let Ok(true) = intcode::process_instruction(&mut prog, &mut pc, &mut rb, &mut intcode::err_input, &mut intcode::stdout_output) {}
    prog[0] == expected
}

fn program(args: &Vec<String>) -> Vec<i64> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

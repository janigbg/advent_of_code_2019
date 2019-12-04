extern crate advent_of_code_2019;

use advent_of_code_2019::{intcode, parser};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let program = program(&args);

    let expected = 19690720;
    let mut found: Option<(i32, i32)> = None;

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

fn test_program(expected: i32, program: &Vec<i32>, noun: i32, verb: i32) -> bool {
    println!("Testing Noun: {}, Verb: {}", noun, verb);
    let mut prog = program.clone();
    prog[1] = noun;
    prog[2] = verb;
    let mut pc = 0;
    while intcode::process_instruction(&mut prog, &mut pc) {}
    prog[0] == expected
}

fn program(args: &Vec<String>) -> Vec<i32> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

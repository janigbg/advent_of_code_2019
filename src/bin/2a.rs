extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use std::env;

#[derive(Debug)]
enum Instruction {
    Sum { i1: usize, i2: usize, out: usize },
    Prod { i1: usize, i2: usize, out: usize },
    Halt,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.clone() {
        println!("{}", arg);
    }

    let mut program = program(args);

    let mut pc = 0;
    while process_instruction(&mut program, &mut pc) {}

    println!("Result: {}", program[0]);
}

fn process_instruction(program: &mut Vec<i32>, pc: &mut usize) -> bool {
    let instruction = match program[*pc] {
        1 => Instruction::Sum {
            i1: program[*pc + 1] as usize,
            i2: program[*pc + 2] as usize,
            out: program[*pc + 3] as usize,
        },
        2 => Instruction::Prod {
            i1: program[*pc + 1] as usize,
            i2: program[*pc + 2] as usize,
            out: program[*pc + 3] as usize,
        },
        99 => Instruction::Halt,
        _ => panic!(format!("Invalid instruction at {}: {}", *pc, program[*pc])),
    };

    println!("{:?}", instruction);

    match instruction {
        Instruction::Sum { i1, i2, out } => {
            program[out] = program[i1] + program[i2];
            *pc += 4;
            true
        }
        Instruction::Prod { i1, i2, out } => {
            program[out] = program[i1] * program[i2];
            *pc += 4;
            true
        }
        Instruction::Halt => {
            *pc += 1;
            false
        }
    }
}

fn program(args: Vec<String>) -> Vec<i32> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

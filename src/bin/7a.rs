extern crate advent_of_code_2019;

use advent_of_code_2019::{intcode, parser};
use permutohedron::LexicalPermutation;
use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let program = program(&args);

    let mut results: Vec<i64> = Vec::new();
    let mut phase_settings = [0, 1, 2, 3, 4];
    let mut iter = true;
    while iter {
        let mut io = IO::new(&phase_settings);
        let result = process_program(&program, &mut io).expect("Program error");
        results.push(result);
        iter = phase_settings.next_permutation();
    }
    
    println!("Result: {}", results.iter().max().unwrap());
}

pub fn process_program(
    program: &Vec<i64>,
    io: &mut IO,
) -> Result<i64, Box<dyn Error>> {
    for _ in 0..5 {
        let mut prog = program.clone();
        let mut pc = 0;
        let mut rb = 0;
        let mut out = None;
        while let true = intcode::process_instruction(&mut prog, &mut pc, &mut rb, &mut || io.input(), &mut |i| out = Some(i))? {
            if out != None {
                io.set_output(out);
                out = None;
            }
        }
    }

    match io.current_out {
        Some(val) => Ok(val),
        None => Err(Box::new(intcode::IntCodeError::from(format!("No output"))))
    }
}

fn program(args: &Vec<String>) -> Vec<i64> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum InputType {
    Phase,
    Signal,
}

#[derive(Debug)]
pub struct IO {
    phase: [i64; 5],
    current: i64,
    input_type: InputType,
    current_out: Option<i64>,
}

impl IO {
    pub fn new(phase: &[i64;5]) -> Self {
        let mut result = IO {
            phase: [0; 5],
            current: -1,
            input_type: InputType::Phase,
            current_out: None,
        };

        result.phase.copy_from_slice(phase);

        result
    }

    pub fn input(&mut self) -> Result<i64, Box<dyn Error>> {
        if self.input_type == InputType::Phase {
            self.current += 1;
        }

        let result: Result<i64, Box<dyn Error>> = match (self.current as usize, self.input_type, self.current_out) {
            (0, InputType::Signal, _) => Ok(0),
            (_, InputType::Signal, Some(out)) => Ok(out),
            (current, InputType::Phase, _) if current < 5 => Ok(self.phase[current]),
            _ => Err(Box::new(intcode::IntCodeError::from(format!("Invalid IO state: {:?}", self)))),
        };

        self.input_type = match self.input_type {
            InputType::Phase => InputType::Signal,
            InputType::Signal => InputType::Phase,
        };

        println!("IO Input: {:?}", result);
        result
    }

    pub fn set_output(&mut self, val: Option<i64>) {
        println!("IO Output: {:?}", val);
        self.current_out = val;
    }
}
extern crate advent_of_code_2019;

use advent_of_code_2019::{intcode, parser};
use crossbeam_channel::{bounded, Receiver, Sender};
use permutohedron::LexicalPermutation;
use std::env;
use std::error::Error;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let program = program(&args);

    let mut results: Vec<i32> = Vec::new();
    let mut phase_settings = [5, 6, 7, 8, 9];
    let mut iter = true;
    while iter {
        let result = process_program(&program, &phase_settings).expect("Program error");
        results.push(result);
        iter = phase_settings.next_permutation();
    }
    
    println!("Result: {}", results.iter().max().unwrap());
}

#[derive(Debug)]
struct IO {
    i : usize,
    tx: Sender<i32>,
    rx: Receiver<i32>,
}

impl IO {
    pub fn new(i: usize, sender: &Sender<i32>, receiver: &Receiver<i32>) -> Self {
        IO {
            i,
            tx: sender.clone(),
            rx: receiver.clone(),
        }
    }

    pub fn input(&self) -> Result<i32, Box<dyn Error>> {
        let x = self.rx.recv()?;
        println!("RECV({}): {}", AMPLIFIER_NAMES[self.i], x);
        Ok(x)
    }

    pub fn output(&self, val: i32) -> () {
        println!("SEND({}): {}", AMPLIFIER_NAMES[self.i], val);
        self.tx.send(val).expect(&format!("Error sending {}", val));
    }
}

const AMPLIFIER_NAMES: [&str; 5] = ["A", "B", "C", "D", "E"];

pub fn process_program(
    program: &Vec<i32>,
    phase_settings: &[i32; 5],
) -> Result<i32, Box<dyn Error>> {

    let channels: [(Sender<i32>, Receiver<i32>); 5] = [bounded(5), bounded(5), bounded(5), bounded(5), bounded(5)];
    let mut children = Vec::new();

    for i in 0..5 {
        let mut prog = program.clone();
        let io = IO::new(i, &channels[(i + 1) % 5].0, &channels[i].1);
        let child = thread::spawn(move || {
            let mut pc = 0;
            while let Ok(true) = intcode::process_instruction(&mut prog, &mut pc, &mut || io.input(), &mut |i| io.output(i)) {}
        });

        children.push(child);
    }

    for (i, p) in phase_settings.iter().enumerate() {
        channels[i].0.send(*p)?;
    }

    channels[0].0.send(0)?;

    for child in children {
        child.join().expect("Child thread has panicked");
    }

    let out = channels[0].1.recv()?;
    println!("OUT: {}", out);

    Ok(out)
}

fn program(args: &Vec<String>) -> Vec<i32> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
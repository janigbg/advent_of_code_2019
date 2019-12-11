extern crate advent_of_code_2019;

use advent_of_code_2019::{intcode, parser};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let mut program = program(&args);

    for _ in 0..1000 {
        program.push(0);
    }

    let (sender, receiver) = bounded(5);
    let map = Map::new();
    let mut io = IO::new(&sender, &receiver, map);

    let mut pc = 0;
    let mut rb = 0;
    while let Ok(true) = intcode::process_instruction(&mut program, &mut pc, &mut rb, &mut || io.input(), &mut |v| io.output(v)) {}

    let result = io.get_colors();

    println!("Done: {}", result);
}

#[derive(Debug, Copy, Clone)]
enum OutputType {
    Color,
    Direction,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Map {
    x: i64,
    y: i64,
    dir: Direction,
    colors: HashMap<(i64, i64), i64>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            x: 0,
            y: 0,
            dir: Direction::Up,
            colors: HashMap::new(),
        }
    }

    pub fn get_color(&self) -> i64 {
        match self.colors.get(&(self.x, self.y)) {
            Some(c) => *c,
            None => 0,
        }
    }

    pub fn get_all_painted(&self) -> usize {
        self.colors.len()
    }
 
    pub fn paint(&mut self, color: i64) {
        self.colors.insert((self.x, self.y), color);
    }

    pub fn turn_and_move(&mut self, dir: i64) {
        self.dir = match (self.dir, dir) {
            (Direction::Up, 0) => Direction::Left,
            (Direction::Up, 1) => Direction::Right,
            (Direction::Right, 0) => Direction::Up,
            (Direction::Right, 1) => Direction::Down,
            (Direction::Down, 0) => Direction::Right,
            (Direction::Down, 1) => Direction::Left,
            (Direction::Left, 0) => Direction::Down,
            (Direction::Left, 1) => Direction::Up,
            _ => self.dir
        };
        match self.dir {
            Direction::Up => {
                self.y -= 1;
            },
            Direction::Right => {
                self.x += 1;
            },
            Direction::Down => {
                self.y += 1;
            },
            Direction::Left => {
                self.x -= 1;
            }
        }
    }
}

#[derive(Debug)]
struct IO {
    tx: Sender<i64>,
    rx: Receiver<i64>,
    next_output: RefCell<OutputType>,
    map: RefCell<Map>,
}

impl IO {
    pub fn new(sender: &Sender<i64>, receiver: &Receiver<i64>, map: Map) -> Self {
        IO {
            tx: sender.clone(),
            rx: receiver.clone(),
            next_output: RefCell::new(OutputType::Color),
            map: RefCell::new(map),
        }
    }

    pub fn input(&self) -> Result<i64, Box<dyn Error>> {
        //let x = self.rx.recv()?;
        let x = self.map.borrow().get_color();
        println!("RECV: {}", x);
        Ok(x)
    }

    pub fn output(&self, val: i64) -> () {
        println!("SEND: {}", val);
        let mut next_out = self.next_output.borrow_mut();
        let mut map = self.map.borrow_mut();
        match *next_out {
            OutputType::Color => {
                map.paint(val);
                *next_out = OutputType::Direction;
            },
            OutputType::Direction => {
                map.turn_and_move(val);
                *next_out = OutputType::Color;
            }
        }
        //self.tx.send(val).expect(&format!("Error sending {}", val));
    }

    pub fn get_colors(&self) -> usize {
        self.map.borrow().get_all_painted()
    } 
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

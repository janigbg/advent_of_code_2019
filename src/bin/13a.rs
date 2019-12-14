extern crate advent_of_code_2019;
extern crate device_query;
extern crate image;

use advent_of_code_2019::{intcode, parser};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::i64;
use std::io;
use std::io::Write;
use std::path::Path;
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let mut program = program(&args);
    for _ in 0..1000000 {
        program.push(0);
    }

    if args.len() > 3 {
        println!("{}", args[3]);
        program[0] = args[3].parse().unwrap();
    }

    let mut io = IO::new();

    let mut pc = 0;
    let mut rb = 0;
    while let Ok(true) = intcode::process_instruction(
        &mut program,
        &mut pc,
        &mut rb,
        &mut || io.input(),
        &mut |v| io.output(v),
    ) {}

    println!("Blocks: {}", io.count_blocks());
    println!("Score: {}", io.get_score());
}

#[derive(Debug, Copy, Clone)]
enum OutputType {
    X,
    Y,
    Tile,
}

struct IO {
    x: RefCell<i64>,
    y: RefCell<i64>,
    score: RefCell<i64>,
    ball_x: RefCell<i64>,
    paddle_x: RefCell<i64>,
    next_output: RefCell<OutputType>,
    map: RefCell<HashMap<(i64, i64), i64>>,
    device: DeviceState,
}

impl IO {
    pub fn new() -> Self {
        IO {
            x: RefCell::new(0),
            y: RefCell::new(0),
            score: RefCell::new(0),
            ball_x: RefCell::new(0),
            paddle_x: RefCell::new(0),
            next_output: RefCell::new(OutputType::X),
            map: RefCell::new(HashMap::new()),
            device: DeviceState::new(),
        }
    }

    pub fn input(&self) -> Result<i64, Box<dyn Error>> {
        self.output_ascii();
        Ok((*self.ball_x.borrow() - *self.paddle_x.borrow()).signum())
    }

    fn _manual_input(&self) -> Result<i64, Box<dyn Error>> {
        let keys: Vec<Keycode> = self.device.get_keys();
        let t = time::Duration::from_millis(500);
        thread::sleep(t);

        if !keys.is_empty() {
            let out: i64 = match keys[0] {
                Keycode::A => -1,
                Keycode::D => 1,
                _ => 0,
            };

            return Ok(out);
        }
        Ok(0)
    }

    pub fn output(&self, val: i64) -> () {
        println!("SEND: {}", val);
        let mut next_out = self.next_output.borrow_mut();
        let mut map = self.map.borrow_mut();
        match *next_out {
            OutputType::X => {
                *self.x.borrow_mut() = val;
                *next_out = OutputType::Y;
            }
            OutputType::Y => {
                *self.y.borrow_mut() = val;
                *next_out = OutputType::Tile;
            }
            OutputType::Tile => {
                match (*self.x.borrow(), val) {
                    (-1, _) => {
                        *self.score.borrow_mut() = val;
                    }
                    (x, 4) => {
                        *self.ball_x.borrow_mut() = x;
                        map.insert((x, *self.y.borrow()), val);
                    }
                    (x, 3) => {
                        *self.paddle_x.borrow_mut() = x;
                        map.insert((x, *self.y.borrow()), val);
                    }
                    (x, _) => {
                        map.insert((x, *self.y.borrow()), val);
                    }
                }
                *next_out = OutputType::X;
            }
        }
    }

    fn get_size(&self) -> (i64, i64, i64, i64) {
        let (x_min, x_max) = self.map.borrow().iter().fold(
            (i64::MAX, i64::MIN),
            |(old_min, old_max), ((x, _), _)| match (*x < old_min, *x > old_max) {
                (true, false) => (*x, old_max),
                (false, true) => (old_min, *x),
                (false, false) => (old_min, old_max),
                (true, true) => (*x, *x),
            },
        );

        let (y_min, y_max) = self.map.borrow().iter().fold(
            (i64::MAX, i64::MIN),
            |(old_min, old_max), ((_, y), _)| match (*y < old_min, *y > old_max) {
                (true, false) => (*y, old_max),
                (false, true) => (old_min, *y),
                (false, false) => (old_min, old_max),
                (true, true) => (*y, *y),
            },
        );

        let x = x_max - x_min + 1;
        let y = y_max - y_min + 1;

        println!("X: {}, Y: {}", x, y);

        (x_min, y_min, x, y)
    }

    pub fn count_blocks(&self) -> usize {
        self.map.borrow().iter().filter(|&(_, t)| *t == 2).count()
    }

    pub fn get_score(&self) -> i64 {
        *self.score.borrow()
    }

    pub fn output_ascii(&self) {
        let (x_min, y_min, width, height) = self.get_size();

        print!("\x1B[2J");
        println!("Score: {}", self.get_score());
        for y in y_min..y_min + height {
            for x in x_min..x_min + width {
                //let tile = self.map.borrow().get(&(x, y));
                let c = match self.map.borrow().get(&(x, y)) {
                    Some(0) => ' ',
                    Some(1) => '#',
                    Some(2) => '*',
                    Some(3) => '-',
                    Some(4) => 'o',
                    _ => ' ',
                };
                print!("{}", c);
            }
            println!();
        }
    }

    pub fn save(&self, path: &Path) {}
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

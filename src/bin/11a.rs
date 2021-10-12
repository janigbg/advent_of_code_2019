extern crate advent_of_code_2019;
extern crate image;

use advent_of_code_2019::{intcode, parser};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::i64;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let mut program = program(&args);
    for _ in 0..1000 {
        program.push(0);
    }

    let mut start_color: Option<i64> = None;
    if args.len() > 3 {
        println!("{}", args[3]);
        start_color = Some(args[3].parse().unwrap());
    }

    let (sender, receiver) = bounded(5);
    let map = Map::new(start_color);
    let io = IO::new(&sender, &receiver, map);

    let mut pc = 0;
    let mut rb = 0;
    while let Ok(true) = intcode::process_instruction(
        &mut program,
        &mut pc,
        &mut rb,
        &mut || io.input(),
        &mut |v| io.output(v),
    ) {}

    io.save(&Path::new("11b.png"));

    let result = io.get_colors();

    println!("Done: {}", result);
}

fn get_rgb_pixel(v: &i64) -> Vec<u8> {
    match *v {
        0 => vec![0, 0, 0],
        1 => vec![255, 255, 255],
        _ => panic!("Invalid pixel"),
    }
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
    pub fn new(start_color: Option<i64>) -> Self {
        let mut result = Map {
            x: 0,
            y: 0,
            dir: Direction::Up,
            colors: HashMap::new(),
        };

        if let Some(col) = start_color {
            result.colors.insert((0, 0), col);
        }

        result
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

    pub fn save(&self, path: &Path) {
        let (x_min, x_max) = self.colors.iter().fold(
            (i64::MAX, i64::MIN),
            |(old_min, old_max), ((x, _), _)| match (*x < old_min, *x > old_max) {
                (true, false) => (*x, old_max),
                (false, true) => (old_min, *x),
                (false, false) => (old_min, old_max),
                (true, true) => (*x, *x),
            },
        );

        let (y_min, y_max) = self.colors.iter().fold(
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

        let mut buffer: Vec<u8> = vec![0_u8; (x * y * 3) as usize];

        println!("Buffer len: {}", buffer.len());

        self.colors
            .iter()
            .map(|((x_old, y_old), color)| ((x_old - x_min, y_old - y_min), get_rgb_pixel(color)))
            .for_each(|((x0, y0), c)| {
                buffer[(y0 * 3 * x + x0 * 3) as usize] = c[0];
                buffer[(y0 * 3 * x + x0 * 3 + 1) as usize] = c[1];
                buffer[(y0 * 3 * x + x0 * 3 + 2) as usize] = c[2];
            });

        if let Err(err) =
            image::save_buffer(path, buffer.as_slice(), x as u32, y as u32, image::RGB(8))
        {
            println!("{}", err);
        }
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
            _ => self.dir,
        };
        match self.dir {
            Direction::Up => {
                self.y -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
            Direction::Down => {
                self.y += 1;
            }
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
            }
            OutputType::Direction => {
                map.turn_and_move(val);
                *next_out = OutputType::Color;
            }
        }
    }

    pub fn get_colors(&self) -> usize {
        self.map.borrow().get_all_painted()
    }

    pub fn save(&self, path: &Path) {
        self.map.borrow().save(path);
    }
}

fn program(args: &Vec<String>) -> Vec<i64> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

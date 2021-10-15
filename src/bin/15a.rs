extern crate advent_of_code_2019;
extern crate image;
extern crate pathfinding;

use advent_of_code_2019::{intcode, parser};
use core::borrow;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::i64;
use std::io;
use std::io::Write;
use std::path::Path;
use std::{thread, time};
use pathfinding::prelude::{absdiff, astar};

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

    let io = IO::new();

    let mut pc = 0;
    let mut rb = 0;
    while let (false, Ok(true)) = (io.is_done(), intcode::process_instruction(
        &mut program,
        &mut pc,
        &mut rb,
        &mut || io.input(),
        &mut |v| io.output(v),
    )) {}

    let start = io.start();
    let goal = io.goal();

    let result = astar(&start, |p| io.successor(p),|p| distance(p, &goal) / 3,
    |p| *p == goal);

    if let Some((_, dist)) = result {
        println!("DISTANCE: {}", dist);
    }
    
    let minutes = io.all_tiles(1).into_iter().filter_map(|pos| {
        let res = astar(&goal, |p| io.successor(p),|p| distance(p, &pos) / 3,
    |p| *p == pos);
        match res {
            Some((_, dist)) => Some(dist),
            None => None,
        }
    }).max().unwrap();

    println!("Complete map is filled with oxygen in {} minutes", minutes);
}

fn distance(p1: &(i64, i64), p2: &(i64, i64)) -> i32 {
    (absdiff(p1.0, p2.0) + absdiff(p1.1, p2.1)) as i32
}

struct IO {
    pos: RefCell<(i64, i64)>,
    dir: RefCell<i64>,
    prev: RefCell<Vec<i64>>,
    map: RefCell<HashMap<(i64, i64), i64>>,
    done: RefCell<bool>,
}

impl IO {
    pub fn new() -> Self {
        let new = IO {
            pos: RefCell::new((0, 0)),
            dir: RefCell::new(0),
            prev: RefCell::new(Vec::new()),
            map: RefCell::new(HashMap::new()),
            done: RefCell::new(false),
        };

        new.map.borrow_mut().insert((0, 0), 3);

        new
    }

    pub fn successor(&self, pos: &(i64, i64)) -> Vec<((i64, i64), i32)> {
        let map = self.map.borrow();
        let mut result = Vec::new();
        if map.contains_key(pos) && map.get(pos) != Some(&0) {
            if map.contains_key(&(pos.0, pos.1 - 1)) && map.get(&(pos.0, pos.1 - 1)) != Some(&0) {
                result.push(((pos.0, pos.1 - 1), 1))
            }
            if map.contains_key(&(pos.0, pos.1 + 1)) && map.get(&(pos.0, pos.1 + 1)) != Some(&0) {
                result.push(((pos.0, pos.1 + 1), 1))
            }
            if map.contains_key(&(pos.0 - 1, pos.1)) && map.get(&(pos.0 - 1, pos.1)) != Some(&0) {
                result.push(((pos.0 - 1, pos.1), 1))
            }
            if map.contains_key(&(pos.0 + 1, pos.1)) && map.get(&(pos.0 + 1, pos.1)) != Some(&0) {
                result.push(((pos.0 + 1, pos.1), 1))
            }
        }

        result
    }

    pub fn start(&self) -> (i64, i64) {
        let map = self.map.borrow();
        let (pos , _) = map.iter().find(|&(_, v)| *v == 3).unwrap();
        *pos
    }

    pub fn goal(&self) -> (i64, i64) {
        let map = self.map.borrow();
        let (pos , _) = map.iter().find(|&(_, v)| *v == 2).unwrap();
        *pos
    }

    pub fn is_done(&self) -> bool {
        *self.done.borrow()
    }

    pub fn all_tiles(&self, with_val: i64) -> Vec<(i64, i64)> {
        self.map.borrow().iter().filter_map(|(k, v)| match *v == with_val { true => Some(k.clone()), false => None}).collect()
    }

    pub fn input(&self) -> Result<i64, Box<dyn Error>> {
        self.output_ascii();
        let dir: Result<i64, Box<dyn Error>> = match self.get_first_unexplored(&self.map.borrow(), &self.pos.borrow()) {
            Some(x) => Ok(x),
            None => {
                match self.prev.borrow_mut().last() {
                    Some(1) => Ok(2),
                    Some(2) => Ok(1),
                    Some(3) => Ok(4),
                    Some(4) => Ok(3),
                    _ => Err(Box::new(intcode::IntCodeError::new("DONE!"))),
                }
            }
        };

        if let Ok(d) = dir {
            *self.dir.borrow_mut() = d;
            println!("TRY MOVE: {}", d);
        }

        dir
    }

    fn get_first_unexplored(&self, map: &HashMap<(i64, i64), i64>, pos: &(i64, i64)) -> Option<i64> {
        if !map.contains_key(&(pos.0, pos.1 - 1)) {
            Some(1)
        } else if !map.contains_key(&(pos.0, pos.1 + 1)) {
            Some(2)
        } else if !map.contains_key(&(pos.0 - 1, pos.1)) {
            Some(3)
        } else if !map.contains_key(&(pos.0 + 1, pos.1)) {
            Some(4)
        } else {
            None
        }
    }

    pub fn output(&self, val: i64) -> () {
        println!("RESP: {}", val);
        match val {
            0 => {
                let (x, y, _) = self.get_pos_in_dir();
                self.map.borrow_mut().insert((x, y), 0);
                println!("WALL AT {}, {}", x, y);
            },
            1 => {
                let (x, y, backtrack) = self.get_pos_in_dir();
                *self.pos.borrow_mut() = (x, y);
                if !backtrack {
                    self.prev.borrow_mut().push(*self.dir.borrow());
                    println!("MOVE TO {}, {}", x, y);
                    self.map.borrow_mut().insert((x, y), 1);
                } else {
                    self.prev.borrow_mut().pop();
                    println!("BACKTRACK TO {}, {}", x, y);
                }
            },
            2 => {
                let (x, y, backtrack) = self.get_pos_in_dir();
                *self.pos.borrow_mut() = (x, y);
                if !backtrack {
                    self.prev.borrow_mut().push(*self.dir.borrow());
                    println!("MOVE TO {}, {}", x, y);
                    self.map.borrow_mut().insert((x, y), 2);
                } else {
                    self.prev.borrow_mut().pop();
                    println!("BACKTRACK TO {}, {}", x, y);
                }
                println!("TARGET FOUND!");
            },
            _ => panic!("Unknown response code!")
        }
    }

    fn get_pos_in_dir(&self) -> (i64, i64, bool) {
        match *self.dir.borrow() {
            1 => (self.pos.borrow().0, self.pos.borrow().1 - 1, self.prev.borrow().last() == Some(&2)),
            2 => (self.pos.borrow().0, self.pos.borrow().1 + 1, self.prev.borrow().last() == Some(&1)),
            3 => (self.pos.borrow().0 - 1, self.pos.borrow().1, self.prev.borrow().last() == Some(&4)),
            4 => (self.pos.borrow().0 + 1, self.pos.borrow().1, self.prev.borrow().last() == Some(&3)),
            _ => (self.pos.borrow().0, self.pos.borrow().1, false),
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

    pub fn output_ascii(&self) {
        let (x_min, y_min, width, height) = self.get_size();

        print!("\x1B[2J");
        for y in y_min..y_min + height {
            for x in x_min..x_min + width {
                let c = match (x == self.pos.borrow().0, y == self.pos.borrow().1, self.map.borrow().get(&(x, y))) {
                    (true, true, _) => 'D',
                    (_, _, Some(0)) => '#',
                    (_, _, Some(1)) => '.',
                    (_, _, Some(2)) => '*',
                    (_, _, Some(3)) => 'X',
                    _ => ' ',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn program(args: &Vec<String>) -> Vec<i64> {
    parser::parse_comma_list(args)
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

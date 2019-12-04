use crate::parser;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq)]
pub struct Position {
    path: i32,
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x, self.y).hash(state);
    }
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Position {
        Position { path: 0, x, y }
    }

    pub fn move_to(&self, m: Move) -> Vec<Position> {
        match m {
            Move::Up(y) => self.iter_y(y),
            Move::Down(y) => self.iter_y(-y),
            Move::Left(x) => self.iter_x(-x),
            Move::Right(x) => self.iter_x(x),
        }
    }

    pub fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn path(&self) -> i32 {
        self.path
    }

    fn iter_x(&self, x: i32) -> Vec<Position> {
        (1..=x.abs())
            .map(|i| Position {
                path: self.path + i,
                x: self.x + (i * x.signum()),
                y: self.y,
            })
            .collect()
    }

    fn iter_y(&self, y: i32) -> Vec<Position> {
        (1..=y.abs())
            .map(|i| Position {
                path: self.path + i,
                x: self.x,
                y: self.y + (i * y.signum()),
            })
            .collect()
    }
}

pub const ORIGIN: Position = Position::new(0, 0);

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

pub fn parse_move<'a, T>(r: T) -> Move
where
    T: AsRef<str> + 'a,
{
    let s = r.as_ref();
    let val = str::parse::<i32>(&s[1..]).unwrap();
    match &s[..1] {
        "U" => Move::Up(val),
        "D" => Move::Down(val),
        "L" => Move::Left(val),
        "R" => Move::Right(val),
        _ => panic!("Not a valid move!: {}", s),
    }
}

pub fn get_wires(lines: Vec<String>) -> Vec<Vec<Position>> {
    if lines.len() != 2 {
        panic!("Need two sets of wires!");
    }

    let moves: Vec<Vec<Move>> = lines
        .iter()
        .map(|l| {
            parser::read_comma_list(String::from(l))
                .into_iter()
                .map(parse_move)
                .collect()
        })
        .collect();

    let coords: Vec<Vec<Position>> = moves
        .iter()
        .map(|ms| {
            ms.iter().fold(Vec::<Position>::new(), |mut l, &m| {
                l.extend_from_slice(&l.last().unwrap_or(&ORIGIN).move_to(m));
                l
            })
        })
        .collect();

    coords
}

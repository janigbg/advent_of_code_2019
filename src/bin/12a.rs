extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use std::env;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
struct Vec3D(i32, i32, i32);

impl Add for Vec3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Vec3D {
    pub fn signum(&self) -> Vec3D {
        Vec3D(self.0.signum(), self.1.signum(), self.2.signum())
    }

    pub fn abs(&self) -> Vec3D {
        Vec3D(self.0.abs(), self.1.abs(), self.2.abs())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let mut moons: Vec<(Vec3D, Vec3D)> = parser::parse_lines(&args)
        .into_iter()
        .map(parse_moon)
        .map(|m| (m, Vec3D(0, 0, 0)))
        .collect();

    let mut steps: usize = args[3].parse().unwrap();

    while steps > 0 {
        moons = moons
            .iter()
            .enumerate()
            .map(|(i, &(m1, v1))| {
                (
                    m1,
                    moons
                        .iter()
                        .enumerate()
                        .filter(|&(j, _)| i != j)
                        .fold(v1, |rv, (_, &(m2, _))| rv + (m2 - m1).signum()),
                )
            })
            .collect();

        moons = moons.iter().map(|&(m, v)| (m + v, v)).collect();

        steps -= 1;
    }

    let energy = moons.iter().fold(0, |e, &m| e + energy(m));

    println!("{:?}", moons);
    println!("Energy: {}", energy);
}

fn energy(moon: (Vec3D, Vec3D)) -> i32 {
    let (m, v) = (moon.0.abs(), moon.1.abs());
    (m.0 + m.1 + m.2) * (v.0 + v.1 + v.2)
}

fn parse_moon(line: String) -> Vec3D {
    let coords: Vec<&str> = line
        .trim_matches(|c| c == ' ' || c == '<' || c == '>')
        .split(',')
        .collect();
    println!("Coords: {:?}", coords);
    let values: Vec<i32> = coords
        .iter()
        .map(|s| s.split_at(s.find('=').unwrap() + 1).1.parse().unwrap())
        .collect();
    println!("Values: {:?}", values);

    Vec3D(values[0], values[1], values[2])
}

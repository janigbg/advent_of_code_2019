extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use num::integer::Integer;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::env;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
struct Vec3D(i32, i32, i32);

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let moons: Vec<(Vec3D, Vec3D)> = parser::parse_lines(&args)
        .into_iter()
        .map(parse_moon)
        .map(|m| (m, Vec3D(0, 0, 0)))
        .collect();

    let iters: Vec<usize> = (0..3)
        .map(|i| {
            moons
                .iter()
                .map(|&(m, v)| (get_axis(m, i), get_axis(v, i)))
                .collect::<Vec<(i32, i32)>>()
        })
        .map(|v| find_repeat_for_axis(&v))
        .collect();

    let total = iters[0].lcm(&iters[1]).lcm(&iters[2]);

    println!("{:?}", iters);
    println!("Total: {}", total);
}

fn get_axis(v: Vec3D, axis: usize) -> i32 {
    match axis {
        0 => v.0,
        1 => v.1,
        2 => v.2,
        _ => panic!("Invalid axis"),
    }
}

fn find_repeat_for_axis(m: &Vec<(i32, i32)>) -> usize {
    let mut steps: usize = 0;

    let mut iters: HashSet<u64> = HashSet::new();
    let mut moons = m.clone();

    loop {
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

        let mut hasher = DefaultHasher::new();
        moons.hash(&mut hasher);
        let hash = hasher.finish();
        if iters.contains(&hash) {
            break;
        }
        iters.insert(hash);
        steps += 1;
    }

    steps
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

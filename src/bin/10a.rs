extern crate advent_of_code_2019;
extern crate num;

use advent_of_code_2019::parser;
use num::integer::Integer;
use std::collections::HashSet;
use std::env;

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapTile {
    Empty = 0,
    Asteroid = 1,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let map: Vec<Vec<MapTile>> = parser::parse_lines(&args)
        .iter()
        .map(|l: &String|
            l.chars().map(|c| match c {
                '.' => MapTile::Empty,
                '#' => MapTile::Asteroid,
                _ => panic!("Invalid map"),
            })
            .collect()
        )
        .collect();

    let (width, height) = (map[0].len(), map.len());

    let result = (0..(width*height))
        .filter(|i| map[i/height][i%height] == MapTile::Asteroid)
        .map(|i|
            (i, (0..(width*height))
                .filter(|&j| j != i && map[j/height][j%height] == MapTile::Asteroid)
                .map(|j| get_direction((i%height) as i32, (i/height) as i32, (j%height) as i32, (j/height) as i32))
                .collect::<HashSet<_>>()
                .len()))
        .max_by_key(|&(_, num)| num)
        .unwrap();

    println!("Result: X:{}, Y:{}, Num:{}" , result.0 % height, result.0 / height, result.1);
}

fn get_direction(x0: i32, y0: i32, x1: i32, y1: i32) -> (i32, i32) {
    match (x1-x0, y1-y0, (x1-x0).gcd(&(y1-y0))) {
        (x_diff, y_diff, 0) => (x_diff.signum(), y_diff.signum()),
        (x_diff, y_diff, gcd) => (x_diff / gcd, y_diff / gcd),
    }
}
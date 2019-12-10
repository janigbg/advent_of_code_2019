extern crate advent_of_code_2019;
extern crate num;

use advent_of_code_2019::parser;
use num::integer::Integer;
use std::collections::HashSet;
use std::env;
use std::f64;

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

    let (_width, height) = (map[0].len(), map.len());

    let result = get_best_position(&map);

    println!("Angle 1, -1: {}", get_angle((1, -1)));
    println!("Angle 1, 1: {}", get_angle((1, 1)));
    println!("Angle -1, 1: {}", get_angle((-1, 1)));
    println!("Angle -1, -1: {}", get_angle((-1, -1)));

    println!("Result: X:{}, Y:{}, Num:{}" , result.0 % height, result.0 / height, result.1);
}

fn get_direction(x0: i32, y0: i32, x1: i32, y1: i32) -> (i32, i32) {
    match (x1-x0, y1-y0, (x1-x0).gcd(&(y1-y0))) {
        (x_diff, y_diff, 0) => (x_diff.signum(), y_diff.signum()),
        (x_diff, y_diff, gcd) => (x_diff / gcd, y_diff / gcd),
    }
}

fn get_angle(dir: (i32, i32)) -> f64 {
    let l = ((dir.0 as f64) * (dir.0 as f64) + (dir.1 as f64) * (dir.1 as f64)).sqrt();
    match (dir.0, dir.1) {
        (0, y_diff) => 90.0 * y_diff as f64 + 90.0,
        (x_diff, 0) => 90.0 * x_diff as f64,
        (x_diff, y_diff) => (x_diff as f64 / l).sin() * (180.0 / f64::consts::PI),
    } 
}

fn get_best_position(map: &Vec<Vec<MapTile>>) -> (usize, usize) {
    let (width, height) = (map[0].len(), map.len());

    (0..(width*height))
        .filter(|i| map[i/height][i%height] == MapTile::Asteroid)
        .map(|i|
            (i, (0..(width*height))
                .filter(|&j| j != i && map[j/height][j%height] == MapTile::Asteroid)
                .map(|j| get_direction((i%height) as i32, (i/height) as i32, (j%height) as i32, (j/height) as i32))
                .collect::<HashSet<_>>()
                .len()))
        .max_by_key(|&(_, num)| num)
        .unwrap()
}
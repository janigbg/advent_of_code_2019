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

    let (width, _height) = (map[0].len(), map.len());

    let result = get_best_position(&map);

    println!("Start at: X:{}, Y:{}" , result.0%width, result.0/width);

    let nth = get_laser_target(&map, result.0, 200).unwrap();

    println!("Result: X:{}, Y:{}" , nth.0, nth.1);
}

fn get_direction(x0: i32, y0: i32, x1: i32, y1: i32) -> (i32, i32) {
    match (x1-x0, y1-y0, (x1-x0).gcd(&(y1-y0))) {
        (x_diff, y_diff, 0) => (x_diff.signum(), y_diff.signum()),
        (x_diff, y_diff, gcd) => (x_diff / gcd, y_diff / gcd),
    }
}

fn get_angle(dir: (i32, i32)) -> f64 {
    let neg_x = match dir.0 < 0 { true => 1.0, false => 0.0 };
    let result = match (dir.0, dir.1) {
        (0, y_diff) => 90.0 * y_diff as f64 + 90.0,
        (x_diff, 0) => (90.0 * x_diff as f64 + 360.0) % 360.0,
        (x_diff, y_diff) => (2.5 * f64::consts::PI + (y_diff as f64 / x_diff as f64).atan() + neg_x * f64::consts::PI) * (180.0 / f64::consts::PI) % 360.0,
    };

    (result*100.0).round()/100.0
}

fn get_laser_target(map: &Vec<Vec<MapTile>>, pos: usize, nth: usize) -> Result<(usize, usize), String> {
    let (width, height) = (map[0].len(), map.len());

    let asteroids: Vec<(f64, i32, (usize, usize))> = (0..(width*height))
        .filter(|&i| i != pos && map[i/width][i%width] == MapTile::Asteroid)
        .map(|i| {
            let x0 = (pos%width) as i32;
            let y0 = (pos/width) as i32;
            let x1 = (i%width) as i32;
            let y1 = (i/width) as i32;
            let angle = get_angle(get_direction(x0, y0, x1, y1));
            let dist = (y1-y0).abs() + (x1-x0).abs();
            (angle, dist, (x1 as usize, y1 as usize))
        })
        .collect();

    let mut result: Vec<(f64, i32, (usize, usize))> = Vec::new();

    let mut angle: f64 = 0.0;
    while result.len() < nth {
        let mut next = asteroids
            .iter()
            .filter(|&(a, _, _)| (*a - (angle.rem_euclid(360.0))).abs() < f64::EPSILON)
            .collect::<Vec<&(f64, i32, (usize, usize))>>();

        next.sort_by_key(|&(_, d, _)| d);

        if next.len() > angle as usize/360 {
            let next_item = next[angle as usize/360].clone();
            println!("{}:\t{:?}", result.len() + 1, next_item);
            result.push(next_item);
        }
        angle = (100.0 * (angle+0.01)).round() / 100.0; 
    }

    Ok(result.last().unwrap().2)
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
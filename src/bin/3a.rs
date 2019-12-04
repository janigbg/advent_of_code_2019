extern crate advent_of_code_2019;

use advent_of_code_2019::{parser, wires};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let lines = parser::parse_lines(&args);

    let coords = wires::get_wires(lines);

    let result = coords[0]
        .iter()
        .filter(|i| coords[1].contains(i))
        .min_by_key(|p| p.dist())
        .unwrap();

    println!(
        "Closest intersection is at {:?}, Manhattan Distance: {}",
        result,
        result.dist()
    );
}

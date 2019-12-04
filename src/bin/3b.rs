use advent_of_code_2019::{parser, wires};
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let lines = parser::parse_lines(&args);

    let coords = wires::get_wires(lines);

    let first: HashMap<_, _> = coords[0].iter().map(|i| (i, i.path())).collect();
    let second: HashMap<_, _> = coords[1].iter().map(|i| (i, i.path())).collect();

    let result = coords[0]
        .iter()
        .filter(|i| coords[1].contains(i))
        .min_by_key(|p| first.get(p).unwrap() + second.get(p).unwrap())
        .unwrap();

    println!(
        "Closest intersection is at {:?}, Path distance: {}",
        result,
        first.get(result).unwrap() + second.get(result).unwrap()
    );
}

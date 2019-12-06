extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let edges: HashMap<_,_> = parser::parse_lines(&args)
        .iter()
        .map(|l: &String| l.split(')').collect())
        .map(|a: Vec<&str>| (String::from(a[1]), String::from(a[0])))
        .collect();
    
    let mut count = 0;
    for key in edges.keys() {
        let mut item = key;
        while edges.contains_key(item) {
            count += 1;
            item = edges.get(item).unwrap();
        }
    }

    println!("{:?}", edges);

    println!("Total number of orbits: {}", count);
}
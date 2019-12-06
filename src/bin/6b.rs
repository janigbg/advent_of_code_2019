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
    
    let you = path(String::from("YOU"), &edges);
    let san = path(String::from("SAN"), &edges);

    let first = you.iter()
        .filter(|i| san.contains(i))
        .collect::<Vec<&String>>()
        .first()
        .expect("Could not find common node")
        .clone();

    println!("YOU: {:?}, SAN: {:?}", you, san);

    let count = you.iter().position(|s| s == first).unwrap() + san.iter().position(|s| s == first).unwrap();

    println!("Path: {}", count);

    //println!("Total number of orbits: {}", count);
}

fn path(key: String, map: &HashMap<String, String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut item = key;
    while map.contains_key(&item) {
        item = map.get(&item).unwrap().to_string();
        result.push(item.clone())
    }

    result
}
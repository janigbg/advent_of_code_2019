extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    if args.len() < 5 {
        println!("Invalid arguments! (use '-f <filename> <x> <y>'");
        std::process::exit(0);
    }

    let digits: Vec<u32> = parser::parse_digits(&args)
        .iter()
        .map(|d| d.parse().unwrap())
        .collect();

    let (x, y): (usize, usize) = (args[3].parse().unwrap(), args[4].parse().unwrap());

    println!("Dimensions: X = {}, Y = {}", x, y);

    if digits.len() % (x*y) != 0 {
        println!("Image size {} is not a multiple of {} (X * Y)", digits.len(), x*y);
        std::process::exit(0);
    }

    let layers: Vec<&[u32]> = digits
        .chunks(x*y)
        .collect();

    let layer_with_most_0s: &[u32] = layers
        .iter()
        .map(|&a| (a.iter().filter(|&v| *v == 0).count(), a))
        .min_by_key(|&(n, _): &(usize, &[u32])| n)
        .unwrap().1;
    
    let ones_and_zeros: (usize, usize) = (count_digit(layer_with_most_0s, 1), count_digit(layer_with_most_0s, 2));

    println!("1s: {}, 2s: {}, 1s x 2s: {}", ones_and_zeros.0, ones_and_zeros.1, ones_and_zeros.0 * ones_and_zeros.1);
}

fn count_digit(a: &[u32], digit: u32) -> usize {
    a.iter().filter(|&v| *v == digit).count()
}
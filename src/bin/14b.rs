extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use advent_of_code_2019::nanofactory::{NanoFactory, Reaction, Term};
use std::collections::{HashSet};
use std::env;

fn main() {
    const ORE: i64 = 1_000_000_000_000;
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let lines = parser::parse_lines(&args);

    let reactions: HashSet<Reaction> = lines.into_iter().map(|l| parse_reaction(l)).collect();

    println!("{:?}", reactions);

    let fuel = Term {
        chemical: String::from("FUEL"),
        amount: 1,
    };

    let factory = NanoFactory {
        reactions: reactions
    };

    let benchmark = factory.produce(1, &fuel, HashSet::new()).unwrap().0;


    let mut ore_left = ORE;
    let mut total_fuel = 0;
    let mut extra = HashSet::new();

    while ore_left > 0 {
        let min_fuel = std::cmp::min(10000, std::cmp::max(ore_left / benchmark, 1));

        let result = factory.produce(min_fuel,  &fuel, extra).unwrap();
        extra = result.1;

        ore_left = ore_left - result.0 as i64;
        if ore_left >= 0 {
            total_fuel = total_fuel + min_fuel;
            if total_fuel % 100_000 == 0 {
                println!("Iteration: {}", total_fuel);
            }
        }
    }
    

    println!("TOTAL FUEL:");
    println!("{:?}", total_fuel);
}

fn parse_term(term: &str) -> Term {
    let (num, chem) = term.split_at(term.find(' ').unwrap());
    Term {
        chemical: String::from(chem.trim()),
        amount: num.parse().unwrap(),
    }
}

fn parse_reaction(line: String) -> Reaction {
    let sides: Vec<&str> = line.splitn(2, " => ").collect();
    Reaction {
        to: parse_term(sides[1]),
        from: sides[0].split(", ").map(|s| parse_term(s)).collect(),
    }
}

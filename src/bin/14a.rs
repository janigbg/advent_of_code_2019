extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use advent_of_code_2019::nanofactory::{NanoFactory, Reaction, Term};
use std::collections::{HashSet};
use std::env;


fn main() {
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

    let result = factory.produce(fuel, HashSet::new()).unwrap();

    println!("RESULT:");
    println!("{:?}", result);
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
        extra: HashSet::new(),
    }
}

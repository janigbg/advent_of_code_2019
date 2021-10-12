extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use permutohedron::LexicalPermutation;
use std::collections::{HashSet};
use std::env;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Ord, Clone)]
struct Term {
    pub chemical: String,
    pub amount: i32,
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.chemical.partial_cmp(&other.chemical)
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.chemical.eq(&other.chemical)
    }
}

impl Hash for Term {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chemical.hash(state);
    }
}

impl Term {
    pub fn take(&mut self, amount: i32) -> i32 {
        if self.amount >= amount {
            self.amount = self.amount - amount;
            0
        } else {
            let missing = amount - self.amount;
            self.amount = 0;
            missing
        }
    }
}

#[derive(Debug, Eq, Clone)]
struct Reaction {
    pub to: Term,
    pub from: Vec<Term>,
    pub extra: HashSet<Term>,
}

impl Hash for Reaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to.hash(state);
    }
}

impl PartialEq for Reaction {
    fn eq(&self, other: &Self) -> bool {
        self.to.eq(&other.to)
    }
}

struct NanoFactory {
    pub reactions: HashSet<Reaction>,
}

impl NanoFactory {
    pub fn produce(&self, target: Term) -> Result<i32, String> {
        let reaction = self.reactions.get(&Reaction {
            to: target.clone(),
            from: Vec::new(),
            extra: HashSet::new()
        });

        println!("{:?}", reaction);

        match reaction {
            Some(r) => self.produce_reaction(1, r, &HashSet::new()).map(|res|res.0),
            None => Err("Not found".to_string())
        }
    }

    fn produce_reaction(&self, times: i32, reaction: &Reaction, extraIn: &HashSet<Term>) -> Result<(i32, HashSet<Term>), String> {
        let mut bestExtra = extraIn.clone();
        let mut bestResult = i32::max_value();

        let mut permutations = reaction.from.clone();
        permutations.reverse();

        let mut iter = true;
        while iter {
            let mut result = 0;
            let mut extra = extraIn.clone();
            for item in permutations.iter() {
                let mut amount = item.amount * times;
                
                let source = self.reactions.get(&Reaction {
                    to: item.clone(),
                    from: Vec::new(),
                    extra: HashSet::new()
                });

                match source {
                    Some(source_reaction) => {
                        if extra.contains(item) {
                            let mut e = extra.take(item).unwrap();
                            amount = e.take(amount);
                            if e.amount > 0 {
                                extra.insert(e);
                            }
                        }
                        let sourceTimes = (amount as f32 / source_reaction.to.amount as f32).ceil() as i32;
                        let excess = sourceTimes * source_reaction.to.amount - amount;
                        let (output, extras) = self.produce_reaction(sourceTimes, source_reaction, &extra)?;
                        extra = extras;
                        extra.replace(Term { chemical: item.chemical.clone(), amount: excess });
                        result = result + output;
                    },
                    None => {
                        return Ok((amount, extra));
                    }
                }
            }
            if result < bestResult {
                bestResult = result;
                bestExtra = extra;
            }
            iter = false;//permutations.next_permutation();
        }

        println!("Cost for {} {} is {}", times * reaction.to.amount, reaction.to.chemical, bestResult);
        println!("Extra: {:?}", bestExtra);

        Ok((bestResult, bestExtra))
    }
}

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

    let result = factory.produce(fuel).unwrap();

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

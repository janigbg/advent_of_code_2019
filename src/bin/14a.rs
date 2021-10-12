extern crate advent_of_code_2019;

use advent_of_code_2019::parser;
use num::integer::Integer;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::env;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Clone)]
struct Term {
    pub chemical: String,
    pub amount: i32,
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct Reaction {
    pub to: Term,
    pub from: HashSet<Term>,
    pub extra: HashSet<Term>,
}

impl Hash for Reaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to.hash(state);
    }
}

impl Reaction {
    pub fn available(&self, chemicals: &HashSet<Term>) -> bool {
        self.from.difference(chemicals).count() == 0
    }

    pub fn replace_with(&self, replace: &Term, with: HashSet<Term>) -> Reaction {
        let mut target = self.clone();
        if target.from.contains(replace) {
            target.from.remove(replace);
            for w in with.into_iter() {
                
            }
        }

        target
    }

    pub fn produce(&self, target: &Term) -> (HashSet<Term>, HashSet<Term>) {
        if *target != self.to {
            return (
                [target.clone()].iter().cloned().collect::<HashSet<Term>>(),
                HashSet::new(),
            );
        }

        let multiplier = target.amount.div_ceil(&self.to.amount);

        let mut extra: HashSet<Term> = self
            .extra
            .iter()
            .cloned()
            .map(|c| Term {
                chemical: c.chemical,
                amount: multiplier * c.amount,
            })
            .collect();

        let extra_target = self.to.amount * multiplier - target.amount;
        if extra_target > 0 {
            extra.insert(Term {
                chemical: target.chemical.clone(),
                amount: extra_target,
            });
        }

        (
            self.from
                .iter()
                .cloned()
                .map(|c| Term {
                    chemical: c.chemical,
                    amount: multiplier * c.amount,
                })
                .collect(),
            extra,
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    let lines = parser::parse_lines(&args);

    let mut reactions: HashSet<Reaction> = lines.into_iter().map(|l| parse_reaction(l)).collect();

    println!("{:?}", reactions);

    let mut mapped: HashSet<Term> = HashSet::with_capacity(reactions.len());
    let ore = Term {
        chemical: String::from("ORE"),
        amount: 1,
    };
    let fuel = Term {
        chemical: String::from("FUEL"),
        amount: 1,
    };
    mapped.insert(ore.clone());

    let basic: Vec<Term> = reactions
        .iter()
        .cloned()
        .filter(|r| r.available(&mapped))
        .map(|r| r.to)
        .collect();

    let mut target = reactions
        .get(&Reaction {
            to: fuel.clone(),
            from: HashSet::new(),
            extra: HashSet::new(),
        })
        .unwrap()
        .clone();

    let to_replace = target.from.clone();

    for c in to_replace.iter() {
        let mut cs: HashSet<Term> = [c.clone()].iter().cloned().collect();
        while !cs.iter().all(|x| basic.contains(x)) {
            for reduce in cs.iter().filter(|x| !basic.contains(x)) {
                cs.remove(reduce);
                let 
            }
        }
    }

    let mut result: HashMap<Term, Reaction> = HashMap::with_capacity(reactions.len());
    let ore_reaction = Reaction {
        from: [ore.clone()].iter().cloned().collect(),
        to: ore.clone(),
        extra: HashSet::new(),
    };
    result.insert(ore.clone(), ore_reaction);

    while reactions.len() > 0 {
        let available: Vec<Reaction> = reactions
            .iter()
            .cloned()
            .filter(|r| r.available(&mapped))
            .collect();

        for r in available.into_iter() {
            let mut ore_amount = 0;
            let mut extras: HashSet<Term> = HashSet::new();
            for t in r.from.iter() {
                let (or, extra) = result.get(t).unwrap().produce(t);
                ore_amount += or.get(&ore).unwrap().amount;
                for e in extra.into_iter() {
                    extras.replace(match extras.get(&e) {
                        Some(ref ee) => Term {
                            chemical: e.chemical,
                            amount: ee.amount + e.amount,
                        },
                        None => e,
                    });
                }
            }

            reactions.remove(&r);

            let new_r = Reaction {
                to: r.to,
                from: [Term {
                    chemical: ore.chemical.clone(),
                    amount: ore_amount,
                }]
                .iter()
                .cloned()
                .collect(),
                extra: extras,
            };
            mapped.insert(new_r.to.clone());
            result.insert(new_r.to.clone(), new_r);
        }
    }

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

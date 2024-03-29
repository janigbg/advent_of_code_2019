use std::collections::{HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Ord, Clone)]
pub struct Term {
    pub chemical: String,
    pub amount: i64,
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
    pub fn take(&mut self, amount: i64) -> i64 {
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
pub struct Reaction {
    pub to: Term,
    pub from: Vec<Term>,
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

pub struct NanoFactory {
    pub reactions: HashSet<Reaction>,
}

impl NanoFactory {
    pub fn produce(&self, amount: i64, target: &Term, extra: HashSet<Term>) -> Result<(i64, HashSet<Term>), String> {
        let reaction = self.reactions.get(&Reaction {
            to: target.clone(),
            from: Vec::new(),
        });

        //println!("{:?}", reaction);

        match reaction {
            Some(r) => self.produce_reaction(amount, r, &extra),
            None => Err("Not found".to_string())
        }
    }

    fn produce_reaction(&self, times: i64, reaction: &Reaction, extra_in: &HashSet<Term>) -> Result<(i64, HashSet<Term>), String> {
        let mut result = 0;
        let mut extra = extra_in.clone();
        for item in reaction.from.iter() {
            let mut amount = item.amount * times;
            
            let source = self.reactions.get(&Reaction {
                to: item.clone(),
                from: Vec::new(),
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

                    let source_times = (amount as f32 / source_reaction.to.amount as f32).ceil() as i64;
                    let excess = source_times * source_reaction.to.amount - amount;
                    let (output, extras) = self.produce_reaction(source_times, source_reaction, &extra)?;
                    extra = extras;
                    if excess > 0 {
                        let mut newExtra = excess;
                        if let Some(prev) = extra.get(item) {
                            newExtra = newExtra + prev.amount;
                        }
                        extra.replace(Term { chemical: item.chemical.clone(), amount: newExtra });
                    }
                    result = result + output;
                },
                None => {
                    //println!("Cost for {} {} is {}", times * reaction.to.amount, reaction.to.chemical, amount);
                    //println!("Extra: {:?}", extra);
                    return Ok((amount, extra));
                }
            }
        }

        //println!("Cost for {} {} is {}", times * reaction.to.amount, reaction.to.chemical, result);
        //println!("Extra: {:?}", extra);

        Ok((result, extra))
    }
}

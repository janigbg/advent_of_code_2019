const PASSWORD_LENGTH: usize = 6;

lazy_static! {
    static ref MAX_PASSWORD: i32 = 10_i32.pow(PASSWORD_LENGTH as u32);
}

#[derive(Debug, Clone)]
pub struct Password {
    digits: [i32; PASSWORD_LENGTH],
}

impl Password {
    pub fn new(value: i32) -> Password {
        let mut result = Password {
            digits: [0; PASSWORD_LENGTH],
        };

        result
            .digits
            .iter_mut()
            .enumerate()
            .for_each(|(i, d)| *d = (value / digit(i)) % 10);

        result
    }

    pub fn value(&self) -> i32 {
        (0..PASSWORD_LENGTH).fold(0, |v, x| v + self.digits[x] * digit(x))
    }

    fn doubles(&self) -> Vec<usize> {
        self.groups(2)
    }

    fn triplets(&self) -> Vec<usize> {
        self.groups(3)
    }

    fn groups(&self, size: usize) -> Vec<usize> {
        self.digits
            .windows(size)
            .enumerate()
            .filter(|(_, x)| x.iter().all(|&y| y == *x.first().unwrap()))
            .map(|(i, _)| i)
            .collect()
    }

    fn increasing(&self) -> bool {
        let mut previous: Option<i32> = None;
        for current in &self.digits {
            match previous {
                Some(prev) if prev > *current => return false,
                _ => previous = Some(*current),
            }
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        !self.doubles().is_empty() && self.increasing()
    }

    pub fn is_strictly_valid(&self) -> bool {
        let doubles = self.doubles();
        let triplets = self.triplets();

        let has_exact_double = doubles
            .iter()
            .any(|&i| triplets.iter().all(|&j| i < j || i > j + 2));

        has_exact_double && self.increasing()
    }

    pub fn next(&self) -> Option<Password> {
        let mut result = self.value() + 1;

        // naive approach
        while !Password::new(result).is_valid() {
            result += 1;

            if result >= *MAX_PASSWORD {
                return None;
            }
        }

        Some(Password::new(result))
    }
}

fn digit(i: usize) -> i32 {
    match i {
        _ if i >= PASSWORD_LENGTH => panic!("Invalid digit!"),
        _ => 10_i32.pow((PASSWORD_LENGTH - i - 1) as u32),
    }
}

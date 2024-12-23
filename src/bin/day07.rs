use anyhow::Error;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

struct Equation {
    test: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn from_str(s: &str) -> Self {
        let (test_str, numbers_str) = s.split_once(": ").unwrap();
        let test = test_str.parse().unwrap();
        let numbers = numbers_str.split(" ").map(|x| x.parse().unwrap()).collect();
        Self { test, numbers }
    }

    fn solvable(&self) -> bool {
        self.numbers
            .iter()
            .fold(HashSet::from([0]), |acc, new| {
                let mut ret = HashSet::new();
                for acc_el in acc {
                    for new_val in [acc_el + new, acc_el * new] {
                        if new_val <= self.test {
                            ret.insert(new_val);
                        }
                    }
                }
                ret
            })
            .contains(&self.test)
    }
}

fn main() -> Result<(), Error> {
    let equations: Vec<_> = read_to_string(Path::new("data/input07.txt"))?
        .lines()
        .map(Equation::from_str)
        .collect();
    println!(
        "{}",
        equations
            .iter()
            .filter(|&equation| equation.solvable())
            .map(|equation| equation.test)
            .sum::<u64>()
    );

    Ok(())
}

use anyhow::Error;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::{Add, Mul};
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

    fn solvable(&self, ops: &[fn(u64, u64) -> u64]) -> bool {
        self.numbers
            .iter()
            .fold(HashSet::from([0]), |acc, new| {
                let mut ret = HashSet::new();
                for acc_el in acc {
                    for op in ops {
                        let new_val = op(acc_el, *new);
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

fn num_digits(mut val: u64) -> u32 {
    let mut ret = 0;
    while val > 0 {
        ret += 1;
        val /= 10;
    }
    ret
}
fn concatenate(a: u64, b: u64) -> u64 {
    a * 10_u64.pow(num_digits(b)) + b
}

fn main() -> Result<(), Error> {
    let equations: Vec<_> = read_to_string(Path::new("data/input07.txt"))?
        .lines()
        .map(Equation::from_str)
        .collect();
    let ops1 = vec![u64::add, u64::mul];
    let ops2 = vec![u64::add, u64::mul, concatenate];
    for ops in [ops1, ops2] {
        println!(
            "{}",
            equations
                .iter()
                .filter(|&equation| equation.solvable(&ops))
                .map(|equation| equation.test)
                .sum::<u64>()
        );
    }

    Ok(())
}

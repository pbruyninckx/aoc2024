use anyhow::Error;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input = parse_input(&read_to_string(Path::new("data/input11.txt"))?)?;

    for num_iterations in [25, 75] {
        println!("{}", solve(&input, num_iterations));
    }

    Ok(())
}

fn num_digits(mut n: u64) -> u32 {
    let mut ret = 0;
    while n > 0 {
        n /= 10;
        ret += 1;
    }
    ret
}

fn split_number(n: u64) -> Option<(u64, u64)> {
    let digits = num_digits(n);
    if digits % 2 == 0 {
        let mask = 10_u64.pow(digits / 2);
        Some((n / mask, n % mask))
    } else {
        None
    }
}

fn blink(n: u64) -> Vec<u64> {
    if n == 0 {
        vec![1]
    } else if let Some((a, b)) = split_number(n) {
        vec![a, b]
    } else {
        vec![n * 2024]
    }
}

fn solve(input: &[u64], num_iterations: u32) -> usize {
    let mut numbers: HashMap<u64, usize> = input.iter().map(|&n| (n, 1)).collect();
    for _ in 0..num_iterations {
        numbers = numbers
            .iter()
            .flat_map(|(&n, &count)| blink(n).iter().map(|&n| (n, count)).collect::<Vec<_>>())
            .fold(HashMap::new(), |mut acc, (n, count)| {
                acc.entry(n).and_modify(|e| *e += count).or_insert(count);
                acc
            })
    }
    numbers.values().sum()
}

fn parse_input(input: &str) -> Result<Vec<u64>, Error> {
    Ok(input
        .split_whitespace()
        .map(|number| number.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?)
}

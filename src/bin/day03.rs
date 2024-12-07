use anyhow::Error;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

fn solve(input: &str) -> i64 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex
        .captures_iter(input)
        .map(|capture| {
            (1..3)
                .map(|i| capture[i].parse::<i64>().unwrap())
                .product::<i64>()
        })
        .sum()
}

fn solve_with_enabling(input: &str) -> i64 {
    let regex = Regex::new(r"(?s)don't\(\).*?(do\(\)|$)").unwrap();
    regex.split(input).map(solve).sum()
}

fn main() -> Result<(), Error> {
    let input: String = read_to_string(Path::new("data/input03.txt"))?;
    println!("{}", solve(&input));
    println!("{}", solve_with_enabling(&input));
    Ok(())
}

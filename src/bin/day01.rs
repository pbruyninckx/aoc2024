use anyhow::Error;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;
use std::path::Path;

fn get_col(input: &[Vec<i32>], i: usize) -> Vec<i32> {
    let mut col: Vec<_> = input.iter().map(|r| r[i]).collect();
    col.sort();
    col
}

fn solve1(col1: &[i32], col2: &[i32]) -> i32 {
    zip(col1, col2).map(|(x, y)| (x - y).abs()).sum()
}

fn solve2(col1: &[i32], col2: &[i32]) -> i32 {
    let counts = col2.iter().fold(HashMap::new(), |mut acc, x| {
        acc.entry(x).and_modify(|c| *c += 1).or_insert(1);
        acc
    });
    col1.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum()
}

fn main() -> Result<(), Error> {
    let input: Vec<_> = read_to_string(Path::new("data/input01.txt"))?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let col1 = get_col(&input, 0);
    let col2 = get_col(&input, 1);
    println!("{}", solve1(&col1, &col2));
    println!("{}", solve2(&col1, &col2));
    Ok(())
}

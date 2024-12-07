use anyhow::Error;
use std::fs::read_to_string;
use std::iter::zip;
use std::path::Path;

fn is_safe(row: &[i32]) -> bool {
    let acceptable = if row.last().unwrap() >= row.first().unwrap() {
        |(a, b): (&i32, &i32)| a < b && *b <= a + 3
    } else {
        |(a, b): (&i32, &i32)| a > b && *b >= a - 3
    };
    zip(row.iter(), row.iter().skip(1)).all(acceptable)
}

fn is_dampened_safe(row: &[i32]) -> bool {
    (0..row.len()).any(|i| {
        let start_iter = row.iter().take(i);
        let end_iter = row.iter().skip(i + 1);
        let sub_row: Vec<i32> = start_iter.chain(end_iter).cloned().collect();
        is_safe(&sub_row)
    })
}

fn solve(input: &[Vec<i32>], safety_fn: fn(&[i32]) -> bool) -> i32 {
    input.iter().filter(|row| safety_fn(row)).count() as i32
}

fn main() -> Result<(), Error> {
    let input: Vec<_> = read_to_string(Path::new("data/input02.txt"))?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    println!("{}", solve(&input, is_safe));
    println!("{}", solve(&input, is_dampened_safe));
    Ok(())
}

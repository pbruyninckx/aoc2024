use anyhow::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input = parse_input(&read_to_string(Path::new("data/input05.txt"))?);
    println!("{}", solve(&input));
    Ok(())
}

struct Input {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn parse_input(input: &str) -> Input {
    let mut iter = input.lines();
    let rules = iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let updates = iter
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    Input { rules, updates }
}

fn valid_ordering(rules: &[(i32, i32)], update: &[i32]) -> bool {
    let page_position: HashMap<_, _> = update.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    rules.iter().all(|&(page1, page2)| {
        let pos1 = page_position.get(&page1);
        let pos2 = page_position.get(&page2);
        pos1.is_none_or(|p1| pos2.is_none_or(|p2| p1 < p2))
    })
}

fn solve(input: &Input) -> i32 {
    input
        .updates
        .iter()
        .filter(|updates| valid_ordering(&input.rules, updates))
        .map(|update| update[update.len() / 2])
        .sum()
}

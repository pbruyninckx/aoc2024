use anyhow::Error;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input = parse_input(&read_to_string(Path::new("data/input05.txt"))?);
    let (solution1, solution2) = solve(&input);
    println!("{}\n{}", solution1, solution2);
    Ok(())
}

struct Input {
    rules: HashSet<(i32, i32)>,
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

fn valid_ordering(rules: &HashSet<(i32, i32)>, update: &[i32]) -> bool {
    let page_position: HashMap<_, _> = update.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    rules.iter().all(|&(page1, page2)| {
        let pos1 = page_position.get(&page1);
        let pos2 = page_position.get(&page2);
        pos1.is_none_or(|p1| pos2.is_none_or(|p2| p1 < p2))
    })
}

fn middle_update(rules: &HashSet<(i32, i32)>, updates: &[i32]) -> i32 {
    let mut sorted_updates = updates.to_vec();
    sorted_updates.sort_by(|&a, &b| {
        if rules.contains(&(a, b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    sorted_updates[sorted_updates.len() / 2]
}

fn solve(input: &Input) -> (i32, i32) {
    input
        .updates
        .iter()
        .map(|update| {
            if valid_ordering(&input.rules, update) {
                (update[update.len() / 2], 0)
            } else {
                (0, middle_update(&input.rules, update))
            }
        })
        .fold((0, 0), |(a1, a2), (v1, v2)| (a1 + v1, a2 + v2))
}

mod utils;

use crate::utils::map::Pos;
use anyhow::Error;
use std::fs::read_to_string;
use std::path::Path;
use utils::map::Map;

fn main() -> Result<(), Error> {
    let input = Map::<char>::from_str(&read_to_string(Path::new("data/input12.txt"))?)?;

    println!("{}", solve(&input));

    Ok(())
}

fn solve(map: &Map<char>) -> u64 {
    let mut processed = Map {
        size: map.size,
        data: vec![vec![false; map.size.x as usize]; map.size.y as usize],
    };

    let mut result = 0;

    for y in 0..map.size.y {
        for x in 0..map.size.x {
            let pos = Pos { x, y };
            if processed[&pos] {
                continue;
            }
            result += compute_price(map, pos, &mut processed);
        }
    }

    result
}

fn compute_price(map: &Map<char>, start: Pos, processed: &mut Map<bool>) -> u64 {
    let mut to_process = vec![start];
    let mut area = 0;
    let mut border = 0;
    let val = map[&start];
    while let Some(current) = to_process.pop() {
        if processed[&current] {
            continue;
        }
        area += 1;
        for neighbour in map.neighbors(&current) {
            if map[&neighbour] == val {
                to_process.push(neighbour);
            } else {
                border += 1;
            }
        }
        if current.x == 0 || current.x == map.size.x - 1 {
            border += 1;
        }
        if current.y == 0 || current.y == map.size.y - 1 {
            border += 1;
        }
        processed[&current] = true;
    }
    area * border
}

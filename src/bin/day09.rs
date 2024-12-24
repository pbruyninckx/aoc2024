use anyhow::Error;
use itertools::repeat_n;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input = parse_input(&read_to_string(Path::new("data/input09.txt"))?);
    println!("{}", solve(&input));
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Block {
    Id(u64),
    Empty,
}

fn solve(input: &[u64]) -> u64 {
    let mut blocks: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(index, val)| {
            if index % 2 == 0 {
                repeat_n(Block::Id((index / 2) as u64), *val as usize)
            } else {
                repeat_n(Block::Empty, *val as usize)
            }
        })
        .collect();
    let mut index = 0_usize;
    let mut rindex = blocks.len() - 1;
    loop {
        while matches!(blocks[index], Block::Id(_)) {
            index += 1;
        }
        while matches!(blocks[rindex], Block::Empty) {
            rindex -= 1;
        }
        if rindex <= index {
            break;
        }
        blocks[index] = blocks[rindex];
        blocks[rindex] = Block::Empty;
    }
    while matches!(blocks[index], Block::Id(_)) {
        index += 1;
    }
    blocks.truncate(index);
    blocks
        .iter()
        .enumerate()
        .map(|(index, block)| {
            if let Block::Id(id) = block {
                (index as u64) * id
            } else {
                0
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

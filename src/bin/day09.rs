use anyhow::Error;
use itertools::{repeat_n, Itertools};
use std::cmp::{PartialEq, Reverse};
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input = parse_input(&read_to_string(Path::new("data/input09.txt"))?);
    println!("{}", solve(&input));
    println!("{}", solve2(&input));
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Block {
    Id(u64),
    Empty,
}

fn blocks_from_input(input: &[u64]) -> Vec<Block> {
    input
        .iter()
        .enumerate()
        .flat_map(|(index, val)| {
            if index % 2 == 0 {
                repeat_n(Block::Id((index / 2) as u64), *val as usize)
            } else {
                repeat_n(Block::Empty, *val as usize)
            }
        })
        .collect()
}

fn checksum(blocks: &[Block]) -> u64 {
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

fn solve(input: &[u64]) -> u64 {
    let mut blocks = blocks_from_input(input);
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
    checksum(&blocks)
}

fn solve2(input: &[u64]) -> u64 {
    let mut blocks = blocks_from_input(input);
    let mut empty = vec![BinaryHeap::new(); 10];
    blocks
        .iter()
        .enumerate()
        .chunk_by(|(_, block)| matches!(block, Block::Empty))
        .into_iter()
        .filter_map(|(is_empty, mut chunk)| {
            if is_empty {
                let first_index = chunk.next().unwrap().0;
                let last_index = chunk.last().unwrap_or((first_index, &Block::Empty)).0;
                Some(((last_index - first_index + 1) as u64, first_index as u64))
            } else {
                None
            }
        })
        .for_each(|(empty_spaces, first_index)| {
            empty[empty_spaces as usize].push(Reverse(first_index));
        });
    let mut rindex = blocks.len() - 1;
    let mut id = *blocks.last().unwrap();
    assert!(matches!(id, Block::Id(_)));
    while id != blocks[0] {
        while blocks[rindex] != id {
            rindex -= 1;
        }
        let mut number_to_move = 0_usize;
        while blocks[rindex] == id {
            number_to_move += 1;
            rindex -= 1
        }
        if let Some((&Reverse(destination_index), empty_index)) = empty
            .iter()
            .enumerate()
            .skip(number_to_move)
            .filter_map(|(empy_ind, heap)| heap.peek().map(|index| (index, empy_ind)))
            .max()
        {
            if destination_index < rindex as u64 {
                rindex += 1;
                blocks[rindex..rindex + number_to_move].fill(Block::Empty);
                blocks[destination_index as usize..destination_index as usize + number_to_move]
                    .fill(id);
                empty[empty_index].pop();
                if empty_index != number_to_move {
                    empty[empty_index - number_to_move]
                        .push(Reverse(destination_index + number_to_move as u64));
                }
            }
        }
        if let Block::Id(id_number) = id {
            id = Block::Id(id_number - 1);
        }
    }
    checksum(&blocks)
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

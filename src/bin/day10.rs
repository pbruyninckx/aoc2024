use anyhow::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Index;
use std::path::Path;

fn main() -> Result<(), Error> {
    let map = Map::from_str(&read_to_string(Path::new("data/input10.txt"))?)?;
    for score_fn in [trailhead_score, trailhead_rating] {
        println!("{}", solve(&map, score_fn));
    }

    Ok(())
}

fn solve(map: &Map, score_fn: fn(&Map, &Pos) -> u32) -> u32 {
    let start_points = map.data.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &height)| {
            if height == 0 {
                Some(Pos {
                    x: x as i32,
                    y: y as i32,
                })
            } else {
                None
            }
        })
    });

    start_points.map(|pos| score_fn(map, &pos)).sum()
}

fn trailhead_score(map: &Map, start: &Pos) -> u32 {
    let mut positions = vec![*start];
    for i in 1..10 {
        positions = positions
            .iter()
            .flat_map(|pos| map.neighbors(pos))
            .unique()
            .filter(|pos| map[pos] == i)
            .collect();
    }

    positions.len() as u32
}

fn trailhead_rating(map: &Map, start: &Pos) -> u32 {
    let mut positions: HashMap<Pos, u32> = HashMap::from([(*start, 1)]);
    for i in 1..10 {
        positions = positions
            .iter()
            .flat_map(|(pos, &count)| {
                map.neighbors(pos)
                    .iter()
                    .map(move |neighbor| (*neighbor, count))
                    .collect::<Vec<_>>()
            })
            .filter(|(pos, _)| map[pos] == i)
            .fold(HashMap::new(), |mut acc, (pos, count)| {
                acc.entry(pos).and_modify(|c| *c += count).or_insert(count);
                acc
            });
    }

    positions.values().sum()
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

struct Map {
    size: Pos,
    data: Vec<Vec<u8>>,
}

impl Map {
    fn from_str(s: &str) -> Result<Self, Error> {
        let data: Vec<Vec<u8>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|d| d as u8)
                            .ok_or(Error::msg("non-digit found in input"))
                    })
                    .collect()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let size = Pos {
            x: data.first().ok_or(Error::msg("Empty data"))?.len() as i32,
            y: data.len() as i32,
        };
        Ok(Self { data, size })
    }
    fn contains(&self, pos: &Pos) -> bool {
        0 <= pos.x && pos.x < self.size.y && 0 <= pos.y && pos.y < self.size.y
    }

    fn neighbors(&self, pos: &Pos) -> Vec<Pos> {
        [-1, 1]
            .iter()
            .flat_map(|i| {
                [
                    Pos {
                        x: pos.x + i,
                        y: pos.y,
                    },
                    Pos {
                        x: pos.x,
                        y: pos.y + i,
                    },
                ]
            })
            .filter(|pos| self.contains(pos))
            .collect()
    }
}

impl Index<&Pos> for Map {
    type Output = u8;

    fn index(&self, pos: &Pos) -> &Self::Output {
        &self.data[pos.y as usize][pos.x as usize]
    }
}

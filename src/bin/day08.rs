use anyhow::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::{Add, Div, Mul, Sub};
use std::path::Path;

fn main() -> Result<(), Error> {
    let map = Map::from_str(&read_to_string(Path::new("data/input08.txt"))?);

    [get_antinodes, get_resonant_antinodes]
        .iter()
        .for_each(|f| println!("{}", solve(&map, *f)));

    Ok(())
}

fn solve(map: &Map, antinode_fn: fn(&[Pos], &Map) -> Vec<Pos>) -> usize {
    map.antennas
        .iter()
        .flat_map(|(_ch, positions)| antinode_fn(positions, map))
        .unique()
        .count()
}

fn get_antinodes(positions: &[Pos], map: &Map) -> Vec<Pos> {
    positions
        .iter()
        .combinations(2)
        .flat_map(|pos_combo| {
            let [p1, p2] = pos_combo[..] else {
                panic!("Should never happen")
            };
            let diff = *p2 - *p1;
            vec![*p1 - diff, *p2 + diff]
        })
        .filter(|pos| map.contains(pos))
        .collect()
}

fn get_resonant_antinodes(positions: &[Pos], map: &Map) -> Vec<Pos> {
    positions
        .iter()
        .combinations(2)
        .flat_map(|pos_combo| {
            let [p1, p2] = pos_combo[..] else {
                panic!("Should never happen")
            };
            let diff = {
                let diff = *p2 - *p1;
                let diff_gcd = gcd(diff.x, diff.y);
                diff / diff_gcd
            };
            (0..)
                .map(move |i| *p1 + diff * i)
                .take_while(|pos| map.contains(pos))
                .chain(
                    (1..)
                        .map(move |i| *p1 - diff * i)
                        .take_while(|pos| map.contains(pos)),
                )
        })
        .filter(|pos| map.contains(pos))
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output {
        Pos {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i32> for Pos {
    type Output = Pos;

    fn div(self, rhs: i32) -> Self::Output {
        Pos {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Debug)]
struct Map {
    size: Pos,
    antennas: HashMap<char, Vec<Pos>>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        let size = Pos {
            x: lines[0].chars().count() as i32,
            y: lines.len() as i32,
        };
        let antennas = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, ch)| {
                    if ch == '.' {
                        None
                    } else {
                        Some((
                            Pos {
                                x: x as i32,
                                y: y as i32,
                            },
                            ch,
                        ))
                    }
                })
            })
            .fold(HashMap::new(), |mut antennas, (pos, ch)| {
                antennas
                    .entry(ch)
                    .and_modify(|e: &mut Vec<Pos>| e.push(pos))
                    .or_insert(vec![pos]);
                antennas
            });
        Self { size, antennas }
    }

    fn contains(&self, pos: &Pos) -> bool {
        0 <= pos.x && pos.x < self.size.y && 0 <= pos.y && pos.y < self.size.y
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

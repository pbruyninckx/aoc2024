use crate::utils::map::{ConvertibleFromChar, Map, Pos};
use anyhow::Error;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

mod utils;

fn main() -> Result<(), Error> {
    let (map, start, end) = parse_input(&read_to_string(Path::new("data/input16.txt"))?)?;

    println!("{}", solve(&map, start, end));
    Ok(())
}

fn solve(map: &Map<Tile>, start: Pos, end: Pos) -> i64 {
    let mut heap = BinaryHeap::new();
    let start_state = State {
        pos: start,
        direction: Direction::Right,
    };
    heap.push(ScoredState {
        state: start_state,
        score: 0,
    });
    let mut seen = HashSet::new();

    while let Some(scored_state) = heap.pop() {
        if scored_state.state.pos == end {
            return scored_state.score;
        }
        seen.insert(scored_state.state);

        {
            let next_position = scored_state.state.direction.move_(&scored_state.state.pos);
            let next_state = State {
                pos: next_position,
                direction: scored_state.state.direction,
            };
            if map[&next_position] == Tile::Floor && !seen.contains(&next_state) {
                heap.push(ScoredState {
                    score: scored_state.score + 1,
                    state: next_state,
                })
            }
        }
        for next_direction in &scored_state.state.direction.neighbours() {
            let next_pos = next_direction.move_(&scored_state.state.pos);
            let next_state = State {
                pos: next_pos,
                direction: *next_direction,
            };
            if map[&next_pos] == Tile::Floor && !seen.contains(&next_state) {
                heap.push(ScoredState {
                    state: next_state,
                    score: scored_state.score + 1001,
                });
            }
        }

        while seen.contains(
            &heap
                .peek()
                .expect("loop should break before heap runs empty")
                .state,
        ) {
            heap.pop();
        }
    }
    unreachable!("We should reach the 'E' position")
}

fn parse_input(input: &str) -> Result<(Map<Tile>, Pos, Pos), Error> {
    let map = Map::from_str(input)?;
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    start = Some(Pos {
                        x: x as i64,
                        y: y as i64,
                    })
                }
                'E' => {
                    end = Some(Pos {
                        x: x as i64,
                        y: y as i64,
                    })
                }
                _ => {}
            }
        }
    }

    Ok((
        map,
        start.expect("start tile missing"),
        end.expect("end tile missing"),
    ))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Floor,
    Wall,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn neighbours(&self) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => {
                vec![Direction::Left, Direction::Right]
            }
            Direction::Left | Direction::Right => {
                vec![Direction::Up, Direction::Down]
            }
        }
    }

    fn move_(self, pos: &Pos) -> Pos {
        match self {
            Direction::Left => Pos {
                x: pos.x - 1,
                y: pos.y,
            },
            Direction::Right => Pos {
                x: pos.x + 1,
                y: pos.y,
            },
            Direction::Up => Pos {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::Down => Pos {
                x: pos.x,
                y: pos.y + 1,
            },
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct State {
    pos: Pos,
    direction: Direction,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct ScoredState {
    state: State,
    score: i64,
}

impl PartialOrd<Self> for ScoredState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoredState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Inverse the order, so the heap will work as a min-heap
        other.score.cmp(&self.score)
    }
}

impl ConvertibleFromChar for Tile {
    fn from_char(c: char) -> Result<Tile, Error> {
        match c {
            '.' | 'S' | 'E' => Ok(Tile::Floor),
            '#' => Ok(Tile::Wall),
            _ => Err(Error::msg("Unknown tile character")),
        }
    }
}

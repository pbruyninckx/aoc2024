use anyhow::Error;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::Index;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input = Input::from_str(&read_to_string(Path::new("data/input06.txt"))?);
    println!("{}", solve(&input));
    Ok(())
}

fn solve(input: &Input) -> i32 {
    let mut pos = input.start;
    let mut direction = Direction::Up;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    while input.contains(&pos) {
        (pos, direction) = input.next_state(&pos, direction);
        seen.insert(pos);
    }
    seen.len() as i32
}

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
}

struct Input {
    map: Vec<Vec<Tile>>,
    start: (i32, i32),
    size: (i32, i32),
}

impl Input {
    fn from_str(input: &str) -> Self {
        let mut start = None;
        let map: Vec<_> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Obstacle,
                        '^' => {
                            start = Some((row as i32, col as i32));
                            Tile::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let size = (map.len() as i32, map[0].len() as i32);
        Self {
            map,
            start: start.unwrap(),
            size,
        }
    }

    fn contains(&self, pos: &(i32, i32)) -> bool {
        0 <= pos.0 && pos.0 < self.size.0 && 0 <= pos.1 && pos.1 < self.size.1
    }

    fn next_state(&self, pos: &(i32, i32), direction: Direction) -> ((i32, i32), Direction) {
        let mut direction = direction;
        loop {
            let next_pos = move_(pos, &direction);
            if !self.contains(&next_pos) || self[&next_pos] == Tile::Empty {
                return (next_pos, direction);
            }
            direction = direction.next();
        }
    }
}

impl Index<&(i32, i32)> for Input {
    type Output = Tile;

    fn index(&self, pos: &(i32, i32)) -> &Self::Output {
        &self.map[pos.0 as usize][pos.1 as usize]
    }
}

fn move_(pos: &(i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
    }
}

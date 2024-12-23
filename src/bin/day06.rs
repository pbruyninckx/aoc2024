use anyhow::Error;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::Index;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input = Input::from_str(&read_to_string(Path::new("data/input06.txt"))?);
    let positions = get_positions(&input);
    println!("{}", positions.len());
    println!("{}", solve2(&input, &positions));
    Ok(())
}

fn solve2(input: &Input, positions: &HashSet<Pos>) -> i32 {
    let mut changed_input = input.clone();
    positions
        .iter()
        .filter(|&pos| {
            if *pos != input.start && input.contains(pos) {
                changed_input.map[pos.0 as usize][pos.1 as usize] = Tile::Obstacle;
                let ret = has_loop(&changed_input);
                changed_input.map[pos.0 as usize][pos.1 as usize] = Tile::Empty;
                ret
            } else {
                false
            }
        })
        .count() as i32
}
fn has_loop(input: &Input) -> bool {
    let mut state = State {
        pos: input.start,
        direction: Direction::Up,
    };
    let mut seen: HashSet<State> = HashSet::new();
    loop {
        state = input.next_state(&state);
        if seen.contains(&state) {
            return true;
        }
        if !input.contains(&state.pos) {
            return false;
        }
        seen.insert(state.clone());
    }
}

fn get_positions(input: &Input) -> HashSet<Pos> {
    let mut state = State {
        pos: input.start,
        direction: Direction::Up,
    };
    let mut seen: HashSet<Pos> = HashSet::new();
    while input.contains(&state.pos) {
        state = input.next_state(&state);
        seen.insert(state.pos);
    }
    seen
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Obstacle,
}

type Pos = (i32, i32);

#[derive(Eq, Hash, PartialEq, Clone)]
struct State {
    pos: Pos,
    direction: Direction,
}

#[derive(Clone)]
struct Input {
    map: Vec<Vec<Tile>>,
    start: Pos,
    size: Pos,
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

    fn contains(&self, pos: &Pos) -> bool {
        0 <= pos.0 && pos.0 < self.size.0 && 0 <= pos.1 && pos.1 < self.size.1
    }

    fn next_state(&self, state: &State) -> State {
        let mut direction = state.direction;
        loop {
            let next_pos = move_(&state.pos, &direction);
            if !self.contains(&next_pos) || self[&next_pos] == Tile::Empty {
                return State {
                    pos: next_pos,
                    direction,
                };
            }
            direction = direction.next();
        }
    }
}

impl Index<&Pos> for Input {
    type Output = Tile;

    fn index(&self, pos: &Pos) -> &Self::Output {
        &self.map[pos.0 as usize][pos.1 as usize]
    }
}

fn move_(pos: &Pos, direction: &Direction) -> Pos {
    match direction {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
    }
}

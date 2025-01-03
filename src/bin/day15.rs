use crate::utils::map::{ConvertibleFromChar, Map, Pos};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use regex::Regex;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::path::Path;

mod utils;

fn main() -> Result<(), Error> {
    let (map, start, directions) = parse_input(&read_to_string(Path::new("data/input15.txt"))?)?;

    println!("{}", solve(map.clone(), start, &directions));
    println!(
        "{}",
        solve2(map_to_two(&map), pos_to_two(&start), &directions)
    );
    Ok(())
}

fn pos_to_two(pos: &Pos) -> Pos {
    Pos {
        x: 2 * pos.x,
        y: pos.y,
    }
}

fn map_to_two(map: &Map<Tile>) -> Map<TileTwo> {
    let data = map
        .data
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|t| match t {
                    Tile::Wall => vec![TileTwo::Wall; 2],
                    Tile::Floor => vec![TileTwo::Floor; 2],
                    Tile::Box => vec![TileTwo::BoxLeft, TileTwo::BoxRight],
                })
                .collect()
        })
        .collect();
    let size = pos_to_two(&map.size);
    Map { data, size }
}

fn can_move(map: &Map<Tile>, pos: &Pos, dir: &Direction) -> bool {
    let mut current_pos = *pos;
    while map[&current_pos] != Tile::Wall {
        current_pos = dir.move_(&current_pos);
        if map[&current_pos] == Tile::Floor {
            return true;
        }
    }
    false
}

fn move_(map: &mut Map<Tile>, pos: &Pos, direction: &Direction) -> Pos {
    let mut current_pos = direction.move_(pos);
    if map[&current_pos] == Tile::Floor {
        return current_pos;
    }
    while map[&current_pos] == Tile::Box {
        current_pos = direction.move_(&current_pos);
    }
    map[&current_pos] = Tile::Box;
    map[&direction.move_(pos)] = Tile::Floor;
    direction.move_(pos)
}

fn solve(mut map: Map<Tile>, mut pos: Pos, directions: &[Direction]) -> i64 {
    for d in directions {
        if can_move(&map, &pos, d) {
            pos = move_(&mut map, &pos, d);
        }
    }
    score(&map)
}

fn can_move2(map: &Map<TileTwo>, pos: &Pos, dir: &Direction) -> bool {
    let next = dir.move_(pos);
    match map[&next] {
        TileTwo::Wall => false,
        TileTwo::Floor => true,
        TileTwo::BoxRight => {
            can_move2(map, &next, dir)
                && (dir.horizontal() || can_move2(map, &(next + Pos { x: -1, y: 0 }), dir))
        }
        TileTwo::BoxLeft => {
            can_move2(map, &next, dir)
                && (dir.horizontal() || can_move2(map, &(next + Pos { x: 1, y: 0 }), dir))
        }
    }
}

fn move2(map: &mut Map<TileTwo>, pos: &Pos, direction: &Direction) -> Pos {
    let next = direction.move_(pos);
    match map[&next] {
        TileTwo::Wall => {
            panic!("Method should only be called when moving is possible")
        }
        TileTwo::Floor => {}
        TileTwo::BoxLeft => {
            move2(map, &next, direction);
            if direction.vertical() {
                move2(map, &(next + Pos { x: 1, y: 0 }), direction);
            }
        }
        TileTwo::BoxRight => {
            move2(map, &next, direction);
            if direction.vertical() {
                move2(map, &(next + Pos { x: -1, y: 0 }), direction);
            }
        }
    }
    map[&next] = map[pos];
    map[pos] = TileTwo::Floor;
    next
}

fn solve2(mut map: Map<TileTwo>, mut pos: Pos, directions: &[Direction]) -> i64 {
    for d in directions {
        if can_move2(&map, &pos, d) {
            pos = move2(&mut map, &pos, d);
        }
    }

    score(&map)
}

fn score<T: Score + Copy>(map: &Map<T>) -> i64 {
    map.data
        .iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter().enumerate().map(move |(x, &tile)| {
                if tile.count_for_score() {
                    (100 * y + x) as i64
                } else {
                    0
                }
            })
        })
        .sum()
}

fn parse_input(input: &str) -> Result<(Map<Tile>, Pos, Vec<Direction>), Error> {
    let double_line_break = Regex::new(r"\n\n").unwrap();
    let (map_string, directions_string) = double_line_break
        .splitn(input, 2)
        .collect_tuple()
        .ok_or_else(|| anyhow!("Invalid input format (double line break)"))?;
    let map = Map::from_str(map_string)?;
    let directions: Vec<_> = directions_string
        .chars()
        .filter_map(|c| Direction::from_char(c).ok())
        .collect();
    let start_pos = map_string
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '@' {
                    Some(Pos {
                        x: x as i64,
                        y: y as i64,
                    })
                } else {
                    None
                }
            })
        })
        .next()
        .ok_or(Error::msg("Robot not found"))?;
    Ok((map, start_pos, directions))
}

impl ConvertibleFromChar for Tile {
    fn from_char(c: char) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match c {
            '#' => Ok(Tile::Wall),
            '.' | '@' => Ok(Tile::Floor),
            'O' => Ok(Tile::Box),
            _ => Err(Error::msg("Invalid tile character")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
    Box,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileTwo {
    Wall,
    Floor,
    BoxLeft,
    BoxRight,
}

trait Score {
    fn count_for_score(self) -> bool;
}

impl Score for Tile {
    fn count_for_score(self) -> bool {
        self == Tile::Box
    }
}

impl Score for TileTwo {
    fn count_for_score(self) -> bool {
        self == TileTwo::BoxLeft
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Box => 'O',
        };

        write!(f, "{}", c)?;
        Ok(())
    }
}

impl Debug for TileTwo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            TileTwo::Wall => '#',
            TileTwo::Floor => '.',
            TileTwo::BoxLeft => '[',
            TileTwo::BoxRight => ']',
        };

        write!(f, "{}", c)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, Error> {
        Ok(match c {
            '<' => Direction::Left,
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            _ => Err(Error::msg("Invalid direction"))?,
        })
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

    fn vertical(self) -> bool {
        self == Direction::Up || self == Direction::Down
    }

    fn horizontal(self) -> bool {
        !self.vertical()
    }
}

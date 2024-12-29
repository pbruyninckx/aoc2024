mod utils;

use crate::utils::map::Pos;
use anyhow::Error;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;
use utils::map::Map;

fn main() -> Result<(), Error> {
    let input = Map::<char>::from_str(&read_to_string(Path::new("data/input12.txt"))?)?;

    let (sol1, sol2) = solve(&input);
    println!("{}\n{}", sol1, sol2);

    Ok(())
}

fn solve(map: &Map<char>) -> (u64, u64) {
    let mut processed = Map {
        size: map.size,
        data: vec![vec![false; map.size.x as usize]; map.size.y as usize],
    };

    let mut result1 = 0;
    let mut result2 = 0;

    for y in 0..map.size.y {
        for x in 0..map.size.x {
            let pos = Pos { x, y };
            if processed[&pos] {
                continue;
            }
            let (area, num_borders, num_sides) = compute_price(map, pos, &mut processed);
            result1 += area * num_borders;
            result2 += area * num_sides;
        }
    }

    (result1, result2)
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_direction(pos: &Pos, neighbour: &Pos) -> Direction {
    if pos.x == neighbour.x {
        if pos.y < neighbour.y {
            Direction::Down
        } else {
            Direction::Up
        }
    } else {
        assert_eq!(pos.y, neighbour.y);
        if pos.x < neighbour.x {
            Direction::Right
        } else {
            Direction::Left
        }
    }
}

fn compute_price(map: &Map<char>, start: Pos, processed: &mut Map<bool>) -> (u64, u64, u64) {
    let mut to_process = vec![start];
    let mut area = 0;
    let mut border = 0;
    let mut borders = HashSet::new();
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
                borders.insert((current, get_direction(&current, &neighbour)));
            }
        }
        if current.x == 0 || current.x == map.size.x - 1 {
            border += 1;
            if current.x == 0 {
                borders.insert((current, Direction::Left));
            } else {
                borders.insert((current, Direction::Right));
            }
        }
        if current.y == 0 || current.y == map.size.y - 1 {
            border += 1;
            if current.y == 0 {
                borders.insert((current, Direction::Up));
            } else {
                borders.insert((current, Direction::Down));
            }
        }
        processed[&current] = true;
    }

    let num_sides = get_num_sides(borders);
    (area, border, num_sides)
}

fn get_neighbouring_border((pos, dir): &(Pos, Direction), delta: i32) -> (Pos, Direction) {
    let result_pos = match dir {
        Direction::Up | Direction::Down => Pos {
            x: pos.x + delta,
            y: pos.y,
        },
        Direction::Left | Direction::Right => Pos {
            x: pos.x,
            y: pos.y + delta,
        },
    };
    (result_pos, *dir)
}

fn get_num_sides(mut borders: HashSet<(Pos, Direction)>) -> u64 {
    let mut result = 0;
    while let Some(current) = borders.iter().next().cloned() {
        result += 1;
        for i in 0.. {
            let neighbour = get_neighbouring_border(&current, i);
            if borders.contains(&neighbour) {
                borders.remove(&neighbour);
            } else {
                break;
            }
        }
        for i in 1.. {
            let neighbour = get_neighbouring_border(&current, -i);
            if borders.contains(&neighbour) {
                borders.remove(&neighbour);
            } else {
                break;
            }
        }
    }

    result
}

use crate::utils::map::Pos;
use anyhow::Error;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::fs::read_to_string;
use std::path::Path;

mod utils;

fn main() -> Result<(), Error> {
    let input = Robot::from_multi_str(&read_to_string(Path::new("data/input14.txt"))?)?;

    println!("{}", solve(&input, Pos { x: 101, y: 103 }));

    Ok(())
}

fn get_quadrant(pos: &Pos, size: &Pos) -> Option<usize> {
    let half_width = size.x / 2;
    let half_height = size.y / 2;
    if pos.x == half_width || pos.y == half_height {
        None
    } else {
        let xq = if pos.x > half_width { 1 } else { 0 };
        let yq = if pos.y > half_height { 2 } else { 0 };
        Some(xq + yq)
    }
}

fn solve(robots: &[Robot], size: Pos) -> i32 {
    robots
        .iter()
        .map(|robot| (robot.position + robot.velocity * 100) % size)
        .fold(vec![0; 4], |mut quadrants, position| {
            if let Some(q) = get_quadrant(&position, &size) {
                quadrants[q] += 1
            }
            quadrants
        })
        .iter()
        .product()
}

#[derive(Debug)]
struct Robot {
    position: Pos,
    velocity: Pos,
}

impl Robot {
    fn from_multi_str(input: &str) -> Result<Vec<Self>, Error> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

        RE.captures_iter(input)
            .map(&|caps: Captures| {
                let (_, vals): (&str, [&str; 4]) = caps.extract();
                let [px, py, vx, vy]: [i32; 4] = vals
                    .iter()
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<Vec<i32>, _>>()?
                    .try_into()
                    .unwrap();

                Ok(Self {
                    position: Pos { x: px, y: py },
                    velocity: Pos { x: vx, y: vy },
                })
            })
            .collect()
    }
}

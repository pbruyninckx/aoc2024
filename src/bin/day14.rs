use crate::utils::map::{Map, Pos};
use anyhow::Error;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

mod utils;

fn main() -> Result<(), Error> {
    let input = Robot::from_multi_str(&read_to_string(Path::new("data/input14.txt"))?)?;

    println!("{}", solve(&input, Pos { x: 101, y: 103 }));
    display_trees(&input, Pos { x: 101, y: 103 });

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

fn solve(robots: &[Robot], size: Pos) -> i64 {
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

fn display_trees(robots: &[Robot], size: Pos) {
    let file = File::create("data/output14.txt").unwrap();
    let mut f = BufWriter::new(file);
    for i in 0..10000 {
        writeln!(&mut f, "Iteration {}", i).unwrap();
        let positions = robots
            .iter()
            .map(|robot| (robot.position + robot.velocity * i) % size)
            .collect::<Vec<_>>();
        display(&mut f, &positions, &size);
    }
}

fn display<T: std::io::Write>(f: &mut BufWriter<T>, positions: &[Pos], size: &Pos) {
    let mut map = Map::<char> {
        size: *size,
        data: vec![vec![' '; size.x as usize]; size.y as usize],
    };
    for pos in positions {
        map.data[pos.y as usize][pos.x as usize] = '#';
    }
    for line in &map.data {
        writeln!(f, "{}", line.iter().collect::<String>()).unwrap();
    }
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
                let [px, py, vx, vy]: [i64; 4] = vals
                    .iter()
                    .map(|s| s.parse::<i64>())
                    .collect::<Result<Vec<i64>, _>>()?
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

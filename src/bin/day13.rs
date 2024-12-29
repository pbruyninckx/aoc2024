use anyhow::Error;
use regex::{Captures, Regex};
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let configs = parse_input(&read_to_string(Path::new("data/input13.txt"))?)?;

    println!("{}", configs.iter().map(solve_brute_force).sum::<i64>());
    Ok(())
}

struct Config {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn solve_brute_force(config: &Config) -> i64 {
    (0..100)
        .flat_map(|a| {
            (0..100).filter_map(move |b| {
                if a * config.ax + b * config.bx == config.px
                    && a * config.ay + b * config.by == config.py
                {
                    Some(3 * a + b)
                } else {
                    None
                }
            })
        })
        .min()
        .unwrap_or(0)
}

fn parse_input(s: &str) -> Result<Vec<Config>, Error> {
    let re = Regex::new(r"(?ms)Button A: X\+(\d+)\D*?(\d+)\D*?(\d+)\D*?(\d+)\D*?(\d+)\D*?(\d+)")?;

    re.captures_iter(s)
        .map(&|caps: Captures| {
            let (_, vals): (&str, [&str; 6]) = caps.extract();
            let [ax, ay, bx, by, px, py]: [i64; 6] = vals
                .iter()
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()?
                .try_into()
                .unwrap();

            Ok(Config {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            })
        })
        .collect()
}
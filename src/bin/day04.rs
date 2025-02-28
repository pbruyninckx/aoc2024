use anyhow::Error;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input: Vec<_> = read_to_string(Path::new("data/input04.txt"))?
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    println!("{}", solve(&input));
    println!("{}", solve_x(&input));
    Ok(())
}

fn get_directions() -> Vec<(isize, isize)> {
    (-1..=1)
        .flat_map(|r| (-1..=1).map(move |c| (r, c)))
        .filter(|(dx, dy)| *dx != 0 || *dy != 0)
        .collect()
}

fn solve(input: &[Vec<char>]) -> usize {
    let nrows = input.len() as isize;
    let ncols = input[0].len() as isize;
    let directions = get_directions();

    (0..nrows)
        .map(|r| {
            (0..ncols)
                .map(|c| {
                    directions
                        .iter()
                        .filter(|&(dr, dc)| {
                            input[r as usize][c as usize] == 'X'
                                && 0 <= r + 3 * dr
                                && r + 3 * dr < nrows
                                && 0 <= c + 3 * dc
                                && c + 3 * dc < ncols
                                && input[(r + dr) as usize][(c + dc) as usize] == 'M'
                                && input[(r + 2 * dr) as usize][(c + 2 * dc) as usize] == 'A'
                                && input[(r + 3 * dr) as usize][(c + 3 * dc) as usize] == 'S'
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}
fn solve_x(input: &[Vec<char>]) -> usize {
    let nrows = input.len();
    let ncols = input[0].len();

    (1..nrows - 1)
        .map(|r| {
            (1..ncols - 1)
                .filter(|&c| {
                    input[r][c] == 'A'
                        && has_xs(input[r - 1][c - 1], input[r + 1][c + 1])
                        && has_xs(input[r - 1][c + 1], input[r + 1][c - 1])
                })
                .count()
        })
        .sum()
}

fn has_xs(a: char, b: char) -> bool {
    a != b && [a, b].iter().all(|&c| c == 'M' || c == 'S')
}

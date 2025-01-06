use anyhow::Error;
use itertools::Itertools;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let (state, program) = parse_input(&read_to_string(Path::new("data/input17.txt"))?)?;
    let final_state = solve(&program, &state);
    println!(
        "{}",
        final_state.output.iter().map(|i| i.to_string()).join(",")
    );
    println!("{}", solve2(&program, &state));
    Ok(())
}

fn solve(program: &[i64], start_state: &State) -> State {
    let mut state = start_state.clone();
    while 0 <= state.ip && state.ip < program.len() as i64 {
        let ip = state.ip as usize;
        get_instruction(program[ip])(program[ip + 1], &mut state);
        state.ip += 2;
    }
    state
}

fn solve2(program: &[i64], start_state: &State) -> i64 {
    let mut result = 0;
    for i in 0..(program.len() / 2) {
        result *= 64;
        for a in 0..63 {
            let state = {
                let mut state = start_state.clone();
                state.a = result + a;
                state
            };
            let output = solve(program, &state).output;
            if output.len() % 2 != 0 {
                continue;
            }
            if output[0] == program[program.len() - 2 * i - 2]
                && output[1] == program[program.len() - 2 * i - 1]
            {
                result += a;
                break;
            }
        }
    }
    result
}

fn combo(operand: i64, state: &State) -> i64 {
    match operand {
        0..=3 => operand,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("Unexpected combo operand"),
    }
}

fn adv(operand: i64, state: &mut State) {
    state.a >>= combo(operand, state);
}

fn bxl(operand: i64, state: &mut State) {
    state.b ^= operand;
}

fn bst(operand: i64, state: &mut State) {
    state.b = combo(operand, state) % 8;
}

fn jnz(operand: i64, state: &mut State) {
    if state.a != 0 {
        state.ip = operand - 2; // Correct for +2 afterward
    }
}
fn bxc(_operand: i64, state: &mut State) {
    state.b ^= state.c;
}
fn out(operand: i64, state: &mut State) {
    let output = combo(operand, state) % 8;
    state.output.push(output);
    // print!("{},", output);
}
fn bdv(operand: i64, state: &mut State) {
    state.b = state.a >> combo(operand, state);
}
fn cdv(operand: i64, state: &mut State) {
    state.c = state.a >> combo(operand, state);
}

fn get_instruction(opcode: i64) -> fn(i64, &mut State) {
    match opcode {
        0 => adv,
        1 => bxl,
        2 => bst,
        3 => jnz,
        4 => bxc,
        5 => out,
        6 => bdv,
        7 => cdv,
        _ => panic!("Unexpected instruction opcode"),
    }
}

fn parse_register(line: &str) -> Result<i64, Error> {
    Ok(line
        .split_whitespace()
        .last()
        .ok_or(Error::msg("empty line"))?
        .parse::<i64>()?)
}

fn parse_input(input: &str) -> Result<(State, Vec<i64>), Error> {
    let mut lines = input.lines();

    let registers: Vec<_> = (0..3)
        .map(|_| parse_register(lines.next().ok_or(Error::msg("Register missing"))?))
        .collect::<Result<Vec<_>, _>>()?;
    lines.next();

    let program = lines
        .next()
        .ok_or(Error::msg("'Program:' missing"))?
        .split_whitespace()
        .nth(1)
        .ok_or(Error::msg("Invalid formatting of program line"))?
        .split(',')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok((
        State::new(registers[0], registers[1], registers[2]),
        program,
    ))
}

#[derive(Debug, Clone)]
struct State {
    a: i64,
    b: i64,
    c: i64,
    ip: i64,
    output: Vec<i64>,
}

impl State {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            output: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let state = State::new(0, 0, 9);
        let program = vec![2, 6];
        assert_eq!(solve(&program, &state).b, 1);
    }

    #[test]
    fn test2() {
        let state = State::new(10, 0, 0);
        let program = vec![5, 0, 5, 1, 5, 4];
        assert_eq!(solve(&program, &state).output, vec![0, 1, 2]);
    }

    #[test]
    fn test3() {
        let state = State::new(2024, 0, 0);
        let program = vec![0, 1, 5, 4, 3, 0];
        let final_state = solve(&program, &state);
        assert_eq!(final_state.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(final_state.a, 0);
    }
}

use std::{env, fs, io};

use logos::{Lexer, Logos};
use regex::Regex;

const MUL: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";

fn part_1(input: &str) -> usize {
    let mul = Regex::new(MUL).unwrap();
    mul.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap())
        .sum()
}

fn mul(lex: &mut Lexer<Instruction>) -> (usize, usize) {
    let mul = Regex::new(MUL).unwrap();
    mul.captures(lex.slice())
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .unwrap()
}

#[derive(Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Instruction {
    #[token("do()")]
    Start,
    #[token("don't()")]
    Stop,
    #[regex(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)", mul)]
    Mul((usize, usize)),
}

struct State {
    enabled: bool,
    sum: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            enabled: true,
            sum: 0,
        }
    }
}

fn part_2(input: &str) -> usize {
    Instruction::lexer(input)
        .fold(State::default(), |mut state, result| {
            match result {
                Ok(instruction) => match instruction {
                    Instruction::Start => state.enabled = true,
                    Instruction::Stop => state.enabled = false,
                    Instruction::Mul((a, b)) if state.enabled => state.sum += a * b,
                    _ => {}
                },
                _ => {}
            }
            state
        })
        .sum
}

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string(env::args().nth(1).expect("input file"))?;

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part_1(r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#),
            161
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            part_2(r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}

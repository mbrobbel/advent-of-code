#![feature(iter_array_chunks)]

use std::{fs, iter, str::SplitAsciiWhitespace};

struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn from_str(input: &str) -> Self {
        let mut input = input.lines().rev();
        let positions = (input.next().unwrap().len() + 1) / 4;
        let mut stacks = vec![Vec::new(); positions];
        input.for_each(|line| {
            line.chars()
                .chain(iter::once(' '))
                .array_chunks::<4>()
                .map(|[_, x, _, _]| x)
                .enumerate()
                .filter(|(_, item)| *item != ' ')
                .for_each(|(pos, item)| {
                    stacks[pos].push(item);
                });
        });
        Self { stacks }
    }
    fn apply(&mut self, instruction: Instruction) {
        (0..instruction.amount).for_each(|_| {
            if let Some(item) = self.stacks[instruction.source].pop() {
                self.stacks[instruction.destination].push(item);
            }
        });
    }
    fn top(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect()
    }
    fn apply_9001(&mut self, instruction: Instruction) {
        let split_index = self.stacks[instruction.source].len() - instruction.amount;
        let mut stack = self.stacks[instruction.source].split_off(split_index);
        self.stacks[instruction.destination].append(&mut stack);
    }
}

struct Instruction {
    amount: usize,
    source: usize,
    destination: usize,
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        let mut input = input.split_ascii_whitespace();
        let parse = |iter: &mut SplitAsciiWhitespace| -> usize {
            iter.next()
                .map(str::parse)
                .and_then(Result::ok)
                .expect("bad input")
        };
        input.next();
        let amount = parse(&mut input);
        input.next();
        let source = parse(&mut input) - 1;
        input.next();
        let destination = parse(&mut input) - 1;
        Self {
            amount,
            source,
            destination,
        }
    }
}

fn parse(input: &str) -> (Crates, impl Iterator<Item = Instruction> + '_) {
    let (crates, instructions) = input.split_once("\n\n").expect("bad input");
    let crates = Crates::from_str(crates);
    let instructions = instructions.lines().map(Instruction::from_str);
    (crates, instructions)
}

fn one(input: &str) -> String {
    let (mut crates, instructions) = parse(input);
    instructions.for_each(|instruction| crates.apply(instruction));
    crates.top()
}

fn two(input: &str) -> String {
    let (mut crates, instructions) = parse(input);
    instructions.for_each(|instruction| crates.apply_9001(instruction));
    crates.top()
}

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input")?;
    println!("1: {}", one(&input));
    println!("2: {}", two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn part_one() {
        assert_eq!(one(INPUT), "CMZ");
    }

    #[test]
    fn part_two() {
        assert_eq!(two(INPUT), "MCD");
    }
}

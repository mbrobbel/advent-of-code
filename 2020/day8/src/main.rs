use logos::Logos;
use std::{collections::HashSet, io::Read};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation {
    Accumulate,
    Jump,
    NoOperation,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let mut lex = Token::lexer(input);
        if let Some(Token::Operation) = lex.next() {
            let operation = match lex.slice() {
                "acc" => Operation::Accumulate,
                "jmp" => Operation::Jump,
                "nop" => Operation::NoOperation,
                _ => panic!("bad input"),
            };
            if let Some(Token::Argument(argument)) = lex.next() {
                Instruction {
                    operation,
                    argument,
                }
            } else {
                panic!("bad input")
            }
        } else {
            panic!("bad input");
        }
    }
}

#[derive(Debug, Default)]
pub struct Console {
    boot_code: Vec<Instruction>,
    program_counter: isize,
    accumulator: isize,
    log: HashSet<usize>,
}

impl Console {
    fn with_boot_code(mut self, boot_code: Vec<Instruction>) -> Self {
        self.boot_code = boot_code;
        self
    }
    fn fix_boot_code(mut self) -> Self {
        // first try jump to nop

        // goto last instruction
        let len = self.boot_code.len();

        // find all jump statements that can reach that instruction - else go up one and retry
        let jumps = self
            .boot_code
            .iter()
            .zip(0..len)
            .filter(
                |(
                    Instruction {
                        operation,
                        argument,
                    },
                    index,
                )| {
                    operation == &Operation::Jump && argument + *index as isize == len as isize
                },
            )
            .collect::<Vec<_>>();

        if jumps.len() == 0 {
            let jumps = self
                .boot_code
                .iter()
                .zip(0..len)
                .filter(
                    |(
                        Instruction {
                            operation,
                            argument,
                        },
                        index,
                    )| {
                        operation == &Operation::Jump
                            && argument + *index as isize == len as isize - 3
                    },
                )
                .collect::<Vec<_>>();
            dbg!(jumps);
        }
        self
    }

    fn run(mut self) -> isize {
        loop {
            if !self.log.insert(self.program_counter as usize)
                || self.program_counter as usize == self.boot_code.len()
            {
                break self.accumulator;
            }
            let Instruction {
                operation,
                argument,
            } = self.boot_code[self.program_counter as usize];

            self.program_counter += match operation {
                Operation::Accumulate => {
                    self.accumulator += argument;
                    1
                }
                Operation::Jump => argument,
                Operation::NoOperation => 1,
            };
        }
    }
}

#[derive(Debug, Logos)]
enum Token {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    #[token("acc")]
    #[token("jmp")]
    #[token("nop")]
    Operation,

    #[regex("[+-][0-9]+", |lex| lex.slice().parse())]
    Argument(isize),
}

fn parse_boot_code(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::parse).collect()
}

fn part_one(input: &str) -> isize {
    Console::default()
        .with_boot_code(parse_boot_code(input))
        .run()
}

fn part_two(input: &str) -> isize {
    Console::default()
        .with_boot_code(parse_boot_code(input))
        .fix_boot_code()
        .run()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    dbg!(part_one(&input));
    dbg!(part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 5);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(INPUT), 8);
    }
}

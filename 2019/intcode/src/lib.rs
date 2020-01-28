use std::{collections::VecDeque, ops::Index, str::FromStr};

#[derive(Copy, Clone, Debug)]
enum Instruction {
    /// 1 [a, b, dest]
    Add(isize, isize, usize),
    /// 2 [a, b, dest]
    Mul(isize, isize, usize),
    /// 3 [dest]
    Input(usize),
    /// 4 [value]
    Output(isize),
    /// 5 [cond, dest]
    JumpIfTrue(isize, usize),
    /// 6 [cond, dest]
    JumpIfFalse(isize, usize),
    /// 7 [a, b, dest]
    LessThan(isize, isize, usize),
    /// 8 [a, b, dest]
    Equals(isize, isize, usize),
    /// 9 [value]
    AdjustBase(isize),
    /// 99
    Halt,
}

impl Instruction {
    fn jump(&self, pc: usize) -> usize {
        match self {
            Instruction::Add(_, _, _)
            | Instruction::Mul(_, _, _)
            | Instruction::LessThan(_, _, _)
            | Instruction::Equals(_, _, _) => pc + 4,
            Instruction::Input(_) | Instruction::Output(_) | Instruction::AdjustBase(_) => pc + 2,
            Instruction::JumpIfFalse(a, dest) => {
                if *a == 0 {
                    *dest
                } else {
                    pc + 3
                }
            }
            Instruction::JumpIfTrue(a, dest) => {
                if *a != 0 {
                    *dest
                } else {
                    pc + 3
                }
            }
            _ => pc,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<isize> for ParameterMode {
    fn from(input: isize) -> Self {
        match input {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Memory {
    data: Vec<isize>,
    pub relative_base: isize,
}

impl Index<usize> for Memory {
    type Output = isize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Memory {
    fn init(program: Program) -> Self {
        let mut data = vec![0; program.len() * 8];
        for (idx, op) in program.iter().enumerate() {
            data[idx] = *op;
        }
        Memory {
            data,
            relative_base: 0,
        }
    }

    fn adjust_base(&mut self, value: isize) {
        self.relative_base += value;
    }

    fn store(&mut self, address: Address, value: isize) {
        self.data[address] = value;
    }

    fn param(&self, value: isize, mode: ParameterMode) -> isize {
        match mode {
            ParameterMode::Position => self.data[value as usize],
            ParameterMode::Immediate => value,
            ParameterMode::Relative => self.data[(value + self.relative_base) as usize],
        }
    }

    fn address(&self, value: isize, mode: ParameterMode) -> usize {
        match mode {
            ParameterMode::Position => value as usize,
            ParameterMode::Immediate => unreachable!(),
            ParameterMode::Relative => (value + self.relative_base) as usize,
        }
    }

    fn decode(&self, address: Address) -> Instruction {
        let opcode = &self.data[address];
        let mem = &self.data[address + 1..];
        let modes = vec![
            (opcode / 100 % 10).into(),
            (opcode / 1000 % 10).into(),
            (opcode / 10000 % 10).into(),
        ];
        let param = |idx| self.param(mem[idx], modes[idx]);
        let address = |idx| self.address(mem[idx], modes[idx]);

        match opcode % 100 {
            1 => Instruction::Add(param(0), param(1), address(2)),
            2 => Instruction::Mul(param(0), param(1), address(2)),
            3 => Instruction::Input(address(0)),
            4 => Instruction::Output(param(0)),
            5 => Instruction::JumpIfTrue(param(0), param(1) as usize),
            6 => Instruction::JumpIfFalse(param(0), param(1) as usize),
            7 => Instruction::LessThan(param(0), param(1), address(2)),
            8 => Instruction::Equals(param(0), param(1), address(2)),
            9 => Instruction::AdjustBase(param(0)),
            99 => Instruction::Halt,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Intcode {
    program_counter: usize,
    pub done: bool,
    pub memory: Memory,
    pub input: VecDeque<isize>,
    pub output: VecDeque<isize>,
}

impl FromStr for Intcode {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Intcode::load(
            input
                .lines()
                .next()
                .unwrap()
                .split(',')
                .filter_map(|x| x.parse::<isize>().ok())
                .collect(),
        ))
    }
}

impl Intcode {
    pub fn load(program: Program) -> Self {
        Intcode {
            program_counter: 0,
            done: false,
            memory: Memory::init(program),
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn run<T: IntoIterator<Item = isize>>(&mut self, input: T) -> &mut Self {
        self.input.extend(input);
        loop {
            let instruction = self.memory.decode(self.program_counter);
            self.program_counter = instruction.jump(self.program_counter);
            match instruction {
                Instruction::Add(a, b, dest) => {
                    self.memory.store(dest, a + b);
                }
                Instruction::Mul(a, b, dest) => {
                    self.memory.store(dest, a * b);
                }
                Instruction::Input(dest) => {
                    self.memory.store(dest, self.input.pop_front().unwrap());
                }
                Instruction::Output(value) => {
                    self.output.push_back(value);
                    break;
                }
                Instruction::LessThan(a, b, dest) => {
                    self.memory.store(dest, (a < b).into());
                }
                Instruction::Equals(a, b, dest) => {
                    self.memory.store(dest, (a == b).into());
                }
                Instruction::AdjustBase(value) => {
                    self.memory.adjust_base(value);
                }
                Instruction::Halt => {
                    self.done = true;
                    break;
                }
                _ => {}
            }
        }
        self
    }

    pub fn wait(&mut self) -> &mut Self {
        let c = self.run(vec![]);
        if c.done {
            c
        } else {
            c.wait()
        }
    }
}

type Program = Vec<isize>;
type Address = usize;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_program_with_input(program: Program, input: isize) -> VecDeque<isize> {
        let mut c = Intcode::load(program);
        c.run(vec![input]);
        c.output
    }

    fn test_program(program: Program) -> isize {
        Intcode::load(program).run(vec![0]).memory[0]
    }

    #[test]
    fn simple_test() {
        assert_eq!(test_program(vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(test_program(vec![2, 3, 0, 3, 99]), 2);
        assert_eq!(test_program(vec![2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(test_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
        assert_eq!(test_program(vec![1002, 4, 3, 4, 33]), 1002);
        assert_eq!(test_program(vec![1101, 100, -1, 4, 0]), 1101);

        assert_eq!(test_program(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]), 3);
        assert_eq!(test_program(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]), 3);
        assert_eq!(test_program(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]), 3);
        assert_eq!(test_program(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]), 3);

        assert_eq!(
            test_program(vec![101, -1, 7, 7, 4, 7, 1105, 11, 0, 99]),
            101
        );

        assert_eq!(
            test_program(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9
            ]),
            3
        );
        assert_eq!(
            test_program(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]),
            3
        );
        assert_eq!(
            test_program_with_input(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                8
            ),
            vec![1000]
        );
        assert_eq!(
            test_program_with_input(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                9
            ),
            vec![1001]
        );

        assert_eq!(
            Intcode::load(vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ])
            .wait()
            .output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
        assert_eq!(
            Intcode::load(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0])
                .wait()
                .output,
            vec![1219070632396864]
        );
        assert_eq!(
            Intcode::load(vec![104, 1125899906842624, 99]).wait().output,
            vec![1125899906842624]
        );
    }

    #[test]
    fn instruction_decode() {
        let instruction = 1002;
        assert_eq!(instruction % 100, 02);
        assert_eq!(instruction / 100 % 10, 0);
        assert_eq!(instruction / 1000 % 10, 1);
    }
}

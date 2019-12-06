#[derive(Debug)]
pub struct Memory {
    data: Vec<isize>,
}

#[derive(Debug)]
enum Instruction {
    // 1 [a, b, dest]
    Add(isize, isize, usize),
    // 2 [a, b, dest]
    Mul(isize, isize, usize),
    // 3 [dest]
    Input(usize),
    // 4 [value]
    Output(isize),
    // 5 [cond, dest]
    JumpIfTrue(isize, usize),
    // 6 [cond, dest]
    JumpIfFalse(isize, usize),
    // 7 [a, b, dest]
    LessThan(isize, isize, usize),
    // 8 [a, b, dest]
    Equals(isize, isize, usize),
    // 99
    Halt,
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<isize> for ParameterMode {
    fn from(input: isize) -> Self {
        match input {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!(),
        }
    }
}

impl ParameterMode {
    fn decode(instruction: isize) -> Vec<ParameterMode> {
        vec![
            (instruction / 100 % 10).into(),
            (instruction / 1000 % 10).into(),
        ]
    }
}

impl Memory {
    fn with_capacity(capacity: usize) -> Self {
        Memory {
            data: vec![0; capacity],
        }
    }

    fn init(&mut self, program: Program) {
        for (i, x) in program.iter().enumerate() {
            self.data[i] = *x;
        }
    }

    fn load(&self, address: Address, len: usize) -> &[isize] {
        &self.data[address..address + len]
    }

    fn store(&mut self, address: Address, value: isize) {
        self.data[address] = value;
    }

    fn get_param(&self, value: isize, mode: ParameterMode) -> isize {
        match mode {
            ParameterMode::Position => self.data[value as usize],
            ParameterMode::Immediate => value,
        }
    }

    fn fetch_instruction(&self, address: Address) -> Instruction {
        let i: isize = self.data[address];
        let mode = ParameterMode::decode(i);
        match i % 100 {
            1 => Instruction::Add(
                self.get_param(self.data[address + 1], mode[0]),
                self.get_param(self.data[address + 2], mode[1]),
                self.data[address + 3] as usize,
            ),
            2 => Instruction::Mul(
                self.get_param(self.data[address + 1], mode[0]),
                self.get_param(self.data[address + 2], mode[1]),
                self.data[address + 3] as usize,
            ),
            3 => Instruction::Input(self.data[address + 1] as usize),
            4 => Instruction::Output(self.get_param(self.data[address + 1], mode[0])),
            5 => Instruction::JumpIfTrue(
                self.get_param(self.data[address + 1], mode[0]),
                self.get_param(self.data[address + 2], mode[1]) as usize,
            ),
            6 => Instruction::JumpIfFalse(
                self.get_param(self.data[address + 1], mode[0]),
                self.get_param(self.data[address + 2], mode[1]) as usize,
            ),
            7 => Instruction::LessThan(
                self.get_param(self.data[address + 1], mode[0]),
                self.get_param(self.data[address + 2], mode[1]),
                self.data[address + 3] as usize,
            ),
            8 => Instruction::Equals(
                self.get_param(self.data[address + 1], mode[0]),
                self.get_param(self.data[address + 2], mode[1]),
                self.data[address + 3] as usize,
            ),
            99 => Instruction::Halt,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Intcode {
    program_counter: usize,
    memory: Memory,
}

impl Intcode {
    pub fn with_capacity(capacity: usize) -> Self {
        Intcode {
            program_counter: 0,
            memory: Memory::with_capacity(((capacity + 3) / 4) * 4),
        }
    }

    pub fn load(mut self, program: Program) -> Self {
        self.memory.init(program);
        self
    }

    pub fn run(mut self, input: isize) -> isize {
        loop {
            match self.memory.fetch_instruction(self.program_counter) {
                Instruction::Add(a, b, dest) => {
                    self.memory.store(dest, a + b);
                    self.program_counter += 4;
                }
                Instruction::Mul(a, b, dest) => {
                    self.memory.store(dest, a * b);
                    self.program_counter += 4;
                }
                Instruction::Input(dest) => {
                    self.memory.store(dest, input);
                    self.program_counter += 2;
                }
                Instruction::Output(value) => {
                    println!("{}", value);
                    self.program_counter += 2;
                }
                Instruction::JumpIfTrue(a, dest) => {
                    if a != 0 {
                        self.program_counter = dest;
                    } else {
                        self.program_counter += 3;
                    }
                }
                Instruction::JumpIfFalse(a, dest) => {
                    if a == 0 {
                        self.program_counter = dest;
                    } else {
                        self.program_counter += 3;
                    }
                }
                Instruction::LessThan(a, b, dest) => {
                    self.memory.store(dest, (a < b).into());
                    self.program_counter += 4;
                }
                Instruction::Equals(a, b, dest) => {
                    self.memory.store(dest, (a == b).into());
                    self.program_counter += 4;
                }
                Instruction::Halt => break,
            }
        }

        self.memory.load(0, 1)[0]
    }
}

type Program = Vec<isize>;
type Address = usize;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_program(program: Program) -> isize {
        test_program_with_input(program, 0)
    }

    fn test_program_with_input(program: Program, input: isize) -> isize {
        Intcode::with_capacity(program.len())
            .load(program)
            .run(input)
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
                9
            ),
            3
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

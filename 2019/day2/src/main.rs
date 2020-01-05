use std::{error::Error, io::Read};

fn part_one(input: &str, noun: usize, verb: usize) -> Vec<usize> {
    let mut program: Vec<usize> = input
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    program[1] = noun;
    program[2] = verb;

    let mut start = 0;
    let mut size = 4;
    loop {
        let op = &program[start..start + size].to_vec();
        match op[0] {
            1 => program[op[3]] = program[op[2]] + program[op[1]],
            2 => program[op[3]] = program[op[2]] * program[op[1]],
            99 => break,
            _ => panic!(),
        };
        start += size;
        if start + size > program.len() {
            size = program.len() - start;
        }
    }

    program
}

fn part_two(input: &str) -> usize {
    let target = 19_690_720;
    for noun in 0..100 {
        for verb in 0..100 {
            let result = part_one(&input, noun, verb)[0];
            if result == target {
                return 100 * noun + verb;
            }
        }
    }
    panic!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("{:?}", part_one(&input, 12, 2));
    println!("{:?}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(part_one("1,0,0,0,99", 0, 0), &[2, 0, 0, 0, 99]);
        assert_eq!(part_one("2,3,0,3,99", 3, 0), &[2, 3, 0, 6, 99]);
        assert_eq!(part_one("2,4,4,5,99,0", 4, 4), &[2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            part_one("1,1,1,4,99,5,6,0,99", 1, 1),
            &[30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}

use aoc_2019_intcode::Intcode;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::Read,
};

fn part_one(program: Vec<isize>, phase_setting: Vec<isize>) -> isize {
    (0..5).fold(0isize, |acc, x| {
        Intcode::load(program.clone())
            .run(vec![phase_setting[x], acc])
            .output[0]
    })
}

fn part_two(program: Vec<isize>, phase_setting: Vec<isize>) -> isize {
    // initialize
    let mut amps: Vec<Intcode> = (0..5)
        .map(|i| {
            let mut c = Intcode::load(program.clone());
            c.input.push_back(phase_setting[i]);
            c
        })
        .collect();

    // Start program
    amps[0].run(vec![0]);

    // Feedback loop
    for i in (0..5).cycle() {
        println!("{:?}", i % 5);
        if let Some(output) = amps[i % 5].output.pop_front() {
            amps[i].run(vec![output]);
        } else {
            break;
        }
    }
    0isize
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let program: Vec<isize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<isize>().ok())
        .collect();

    let mut max = 0;
    let mut set: HashSet<isize> = HashSet::with_capacity(5);
    (0..5).for_each(|i| {
        (0..5).for_each(|j| {
            (0..5).for_each(|k| {
                (0..5).for_each(|l| {
                    (0..5).for_each(|m| {
                        let sequence = vec![i, j, k, l, m];
                        set.clear();
                        set.extend(sequence.clone());
                        if set.len() == 5 {
                            max = part_one(program.clone(), sequence).max(max);
                        }
                    })
                })
            })
        })
    });
    println!("part_one: {:?}", max);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(
            part_one(
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                vec![4, 3, 2, 1, 0]
            ),
            43210
        );
        assert_eq!(
            part_one(
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                vec![0, 1, 2, 3, 4]
            ),
            54321
        );
        assert_eq!(
            part_one(
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                vec![1, 0, 4, 3, 2]
            ),
            65210
        );
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(
            part_two(
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                vec![9, 8, 7, 6, 5]
            ),
            139629729
        );
        assert_eq!(
            part_two(
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                vec![9, 7, 8, 5, 6]
            ),
            18216
        );
    }
}

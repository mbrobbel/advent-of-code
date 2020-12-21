use std::{collections::HashSet, io::Read};

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|input| {
            input
                .lines()
                .map(|x| x.chars())
                .flatten()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|input| {
            let sets = input
                .lines()
                .map(|x| x.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            sets.iter()
                .fold(sets[0].clone(), |acc, x| {
                    x.intersection(&acc).copied().collect::<HashSet<_>>()
                })
                .len()
        })
        .sum()
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

    static INPUT: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 11);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(INPUT), 6);
    }
}

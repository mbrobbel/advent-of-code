#![feature(iter_array_chunks)]

use std::{collections::HashSet, fs};

fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .flat_map(|(one, two)| {
            let set = one.chars().collect::<HashSet<_>>();
            two.chars().find(|item| set.contains(item))
        })
        .map(priority)
        .sum()
}

fn two(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .flat_map(|group| {
            group
                .into_iter()
                .map(str::chars)
                .map(HashSet::<char>::from_iter)
                .reduce(|mut set, items| {
                    set.retain(|item| items.contains(item));
                    set
                })
        })
        .flat_map(IntoIterator::into_iter)
        .map(priority)
        .sum()
}

fn priority(input: char) -> u32 {
    match input {
        'a'..='z' => input as u32 - 96,
        'A'..='Z' => input as u32 - 38,
        _ => panic!("bad input"),
    }
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

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part_one() {
        assert_eq!(one(INPUT), 157);
    }

    #[test]
    fn part_two() {
        assert_eq!(two(INPUT), 70);
    }
}

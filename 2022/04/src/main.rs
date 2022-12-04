#![feature(iter_array_chunks)]

use std::{fs, ops::RangeInclusive};

fn parse(input: &str) -> impl Iterator<Item = [RangeInclusive<usize>; 2]> + '_ {
    input
        .lines()
        .flat_map(|pair| pair.split([',', '-']))
        .map(str::parse)
        .filter_map(Result::ok)
        .array_chunks::<2>()
        .map(|range| range[0]..=range[1])
        .array_chunks::<2>()
}

fn one(input: &str) -> usize {
    parse(input)
        .filter(|[one, two]| {
            one.start() >= two.start() && one.end() <= two.end()
                || two.start() >= one.start() && two.end() <= one.end()
        })
        .count()
}

fn two(input: &str) -> usize {
    parse(input)
        .filter(|[one, two]| one.start().max(two.start()) <= one.end().min(two.end()))
        .count()
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

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn part_one() {
        assert_eq!(one(INPUT), 2);
    }

    #[test]
    fn part_two() {
        assert_eq!(two(INPUT), 4);
    }
}

use std::{cmp::Reverse, collections::BinaryHeap, fs};

fn sum_calories(input: &str) -> usize {
    input
        .split("\n")
        .filter_map(|calories| calories.parse::<usize>().ok())
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split("\n\n").map(sum_calories)
}

fn one(input: &str) -> usize {
    parse(input).max().unwrap_or_default()
}

fn two(input: &str) -> usize {
    parse(input)
        .fold(BinaryHeap::with_capacity(4), |mut max, input| {
            max.push(Reverse(input));
            if max.len() > 3 {
                max.pop();
            }
            max
        })
        .into_iter()
        .map(|rev| rev.0)
        .sum()
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

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn part_one() {
        assert_eq!(one(&INPUT), 24000);
    }

    #[test]
    fn part_two() {
        assert_eq!(two(&INPUT), 45000);
    }
}

use std::{collections::HashMap, env, fs, io};

fn parse(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
}

fn part_1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = parse(input).unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part_2(input: &str) -> u32 {
    let mut map = HashMap::<_, u32>::default();
    parse(input)
        .map(|(left, right)| {
            *map.entry(right).or_insert(0) += 1;
            left
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|id| id * map.get(&id).unwrap_or(&0))
        .sum()
}

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string(env::args().nth(1).expect("input file"))?;

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn example_1() {
        assert_eq!(part_1(INPUT), 11);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(INPUT), 31);
    }
}

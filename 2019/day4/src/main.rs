use std::{error::Error, io::Read};

fn is_valid(input: usize) -> bool {
    let chars: Vec<char> = input.to_string().chars().collect();
    let mut zip = chars.iter().take(5).zip(chars.iter().skip(1));

    chars.len() == 6
        && zip
            .clone()
            .map(|(x, y)| (x.to_digit(10).unwrap(), y.to_digit(10).unwrap()))
            .all(|(x, y)| x <= y)
        && zip.any(|(x, y)| x == y)
}

fn is_valid2(input: usize) -> bool {
    let chars: Vec<char> = input.to_string().chars().collect();
    is_valid(input)
        && chars
            .iter()
            .enumerate()
            .map(|(i, x)| chars.iter().skip(i).take_while(move |y| y == &x).count())
            .fold(Vec::new(), |mut acc, x| {
                if acc.last() == Some(&1) || x == 1 || acc.is_empty() {
                    acc.push(x)
                }
                acc
            })
            .into_iter()
            .any(|x| x == 2)
}

fn part_one(input: &str) -> usize {
    let range: Vec<usize> = input
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    (range[0]..range[1]).filter(|x| is_valid(*x)).count()
}

fn part_two(input: &str) -> usize {
    let range: Vec<usize> = input
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    (range[0]..range[1]).filter(|x| is_valid2(*x)).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("{:?}", part_one(&input));
    println!("{:?}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert!(is_valid(111_111));
        assert!(!is_valid(223_450));
        assert!(!is_valid(123_789));
    }

    #[test]
    fn part_two_examples() {
        assert!(is_valid2(112_233));
        assert!(!is_valid2(123_444));
        assert!(is_valid2(111_122));
    }
}

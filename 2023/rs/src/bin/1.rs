use std::{env, fs, io};

fn first_last_digit<I: DoubleEndedIterator<Item = u32>>(mut digits: I) -> Option<u32> {
    digits
        .next()
        .map(|first| 10 * first + digits.next_back().unwrap_or(first))
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|char| char.to_digit(10)))
        .filter_map(first_last_digit)
        .sum()
}

fn part_2(input: &str) -> u32 {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            line.char_indices().filter_map(|(idx, char)| {
                char.to_digit(10).or_else(|| {
                    DIGITS.into_iter().enumerate().find_map(|(value, digit)| {
                        line[idx..].starts_with(digit).then_some(value as u32 + 1)
                    })
                })
            })
        })
        .filter_map(first_last_digit)
        .sum()
}

fn main() -> Result<(), io::Error> {
    let input = env::args().nth(1).expect("input file");
    let input = fs::read_to_string(input)?;

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        const INPUT: &str = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;
        assert_eq!(part_1(INPUT), 142);
    }

    #[test]
    fn example_2() {
        const INPUT: &str = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;
        assert_eq!(part_2(INPUT), 281);
    }
}

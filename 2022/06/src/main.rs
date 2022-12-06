use std::{collections::HashSet, fs};

fn find_run_position(input: &str, len: usize) -> usize {
    input
        .as_bytes()
        .windows(len)
        .position(|items| items.iter().collect::<HashSet<_>>().len() == len)
        .unwrap_or_default()
        + len
}

fn one(input: &str) -> usize {
    find_run_position(input, 4)
}

fn two(input: &str) -> usize {
    find_run_position(input, 14)
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

    #[test]
    fn part_one() {
        assert_eq!(one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_two() {
        assert_eq!(two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

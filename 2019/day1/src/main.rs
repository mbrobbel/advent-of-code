use std::{error::Error, io::Read};

fn fuel(x: u64) -> u64 {
    let w = (x / 3).saturating_sub(2);
    match w {
        0 => w,
        _ => w + fuel(w),
    }
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|x| x.parse::<u64>().ok())
        .map(|x| x / 3 - 2)
        .sum()
}

fn part_two(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|x| x.parse::<u64>().ok())
        .map(fuel)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(part_one("12"), 2);
        assert_eq!(part_one("14"), 2);
        assert_eq!(part_one("1969"), 654);
        assert_eq!(part_one("100756"), 33583);
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(part_two("14"), 2);
        assert_eq!(part_two("1969"), 966);
        assert_eq!(part_two("100756"), 50346);
    }
}

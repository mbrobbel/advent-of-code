use std::{env, fs, io};

fn parse(input: &str) -> impl Iterator<Item = Vec<usize>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(str::parse)
            .filter_map(Result::ok)
            .collect()
    })
}

struct Report(Vec<usize>);

impl Report {
    fn all_increasing(&self) -> bool {
        self.0.is_sorted_by(|a, b| b > a)
    }

    fn all_decreasing(&self) -> bool {
        self.0.is_sorted_by(|a, b| b < a)
    }

    fn level_diff(&self) -> bool {
        self.0
            .iter()
            .zip(self.0.iter().skip(1))
            .all(|(a, b)| a.abs_diff(*b) <= 3)
    }

    fn is_safe(&self) -> bool {
        (self.all_decreasing() || self.all_increasing()) && self.level_diff()
    }

    fn without(&self, idx: usize) -> Self {
        let mut levels = self.0.clone();
        levels.remove(idx);
        Self(levels)
    }
}

fn part_1(input: &str) -> usize {
    parse(input).map(Report).filter(Report::is_safe).count()
}

fn part_2(input: &str) -> usize {
    parse(input)
        .map(Report)
        .filter(|report| {
            report.is_safe()
                || (0..report.0.len())
                    .map(|idx| report.without(idx))
                    .any(|report| report.is_safe())
        })
        .count()
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

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn example_1() {
        assert_eq!(part_1(INPUT), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(INPUT), 4);
    }
}

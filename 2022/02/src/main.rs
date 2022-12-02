use std::{fs, io::Error};

struct Round(Shape, Shape);

impl Round {
    fn from_str(input: &str) -> Option<Round> {
        let mut parts = input.split_ascii_whitespace();
        parts.next().map(Shape::from_str).and_then(|opponent| {
            parts
                .next()
                .map(Shape::from_str)
                .map(|response| Self(opponent, response))
        })
    }
    fn update(self) -> Self {
        Self(
            self.0,
            match self.1 {
                Shape::Rock => self.0.defeats(),
                Shape::Paper => self.0,
                Shape::Scissors => self.0.defeats().defeats(),
            },
        )
    }
    fn outcome(&self) -> usize {
        if self.0.defeats() == self.1 {
            0
        } else if self.0 == self.1 {
            3
        } else {
            6
        }
    }
    fn total_score(self) -> usize {
        self.1.score() + self.outcome()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_str(input: &str) -> Shape {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("bad input"),
        }
    }
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn defeats(&self) -> Shape {
        match self {
            Self::Rock => Shape::Scissors,
            Self::Paper => Shape::Rock,
            Self::Scissors => Shape::Paper,
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Round> + '_ {
    input.split('\n').filter_map(Round::from_str)
}

fn score(input: impl Iterator<Item = Round>) -> usize {
    input.map(Round::total_score).sum()
}

fn one(input: &str) -> usize {
    score(parse(input))
}

fn two(input: &str) -> usize {
    score(parse(input).map(Round::update))
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input")?;
    println!("1: {}", one(&input));
    println!("2: {}", two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn part_one() {
        assert_eq!(one(INPUT), 15);
    }

    #[test]
    fn part_two() {
        assert_eq!(two(INPUT), 12);
    }
}

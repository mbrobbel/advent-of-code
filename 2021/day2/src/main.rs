use std::{fs, io::Error, ops::AddAssign, str::FromStr};

fn part_one(input: &[Command]) -> usize {
    let Position { horizontal, depth } = input.iter().fold(Position::default(), |mut pos, cmd| {
        pos += cmd;
        pos
    });
    horizontal as usize * depth as usize
}

fn part_two(input: &[Command]) -> usize {
    let Track {
        horizontal, depth, ..
    } = input.iter().fold(Track::default(), |mut pos, cmd| {
        pos += cmd;
        pos
    });
    horizontal as usize * depth as usize
}

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split_ascii_whitespace();
        let direction = input.next();
        let distance = match input.next().map(FromStr::from_str) {
            Some(Ok(distance)) => distance,
            _ => panic!("bad input"),
        };
        Ok(match direction {
            Some("forward") => Self::Forward(distance),
            Some("up") => Self::Up(distance),
            Some("down") => Self::Down(distance),
            _ => panic!("bad input"),
        })
    }
}

#[derive(Default)]
struct Position {
    horizontal: u32,
    depth: u32,
}

impl AddAssign<&Command> for Position {
    fn add_assign(&mut self, rhs: &Command) {
        match rhs {
            Command::Forward(distance) => self.horizontal += distance,
            Command::Up(depth) => self.depth -= depth,
            Command::Down(depth) => self.depth += depth,
        }
    }
}

#[derive(Default)]
struct Track {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl AddAssign<&Command> for Track {
    fn add_assign(&mut self, rhs: &Command) {
        match rhs {
            Command::Forward(distance) => {
                self.horizontal += distance;
                self.depth += self.aim * distance;
            }
            Command::Up(units) => self.aim -= units,
            Command::Down(units) => self.aim += units,
        }
    }
}

fn parse(input: impl AsRef<str>) -> Vec<Command> {
    input
        .as_ref()
        .lines()
        .filter_map(|reading| reading.parse().ok())
        .collect()
}

fn main() -> Result<(), Error> {
    let input = parse(fs::read_to_string("input")?);

    dbg!(part_one(&input));
    dbg!(part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn one() {
        assert_eq!(part_one(&parse(INPUT)), 150);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(&parse(INPUT)), 900);
    }
}

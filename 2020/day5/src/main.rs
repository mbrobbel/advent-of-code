use std::{collections::HashSet, io::Read};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }

    fn parse(input: &str) -> Self {
        let row = input
            .chars()
            .take(7)
            .fold(0..128, |acc, x| {
                let width = acc.end - acc.start;
                match x {
                    'F' => acc.start..acc.start + width / 2,
                    'B' => acc.start + width / 2..acc.end,
                    _ => panic!("bad input"),
                }
            })
            .start;
        let column = input
            .chars()
            .skip(7)
            .take(3)
            .fold(0..8, |acc, x| {
                let width = acc.end - acc.start;
                match x {
                    'R' => acc.start + width / 2..acc.end,
                    'L' => acc.start..acc.start + width / 2,
                    _ => panic!("bad input"),
                }
            })
            .start;
        Self { row, column }
    }
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(Seat::parse)
        .map(|x| x.id())
        .max()
        .unwrap_or(0)
}

fn part_two(input: &str) -> usize {
    let set = input
        .lines()
        .map(Seat::parse)
        .map(|x| x.id())
        .collect::<HashSet<_>>();
    (0..128usize)
        .map(|row| (0..8).map(move |column| row * 8 + column))
        .flatten()
        .collect::<HashSet<_>>()
        .difference(&set)
        .copied()
        .filter(|id| (set.contains(&(id + 1)) && set.contains(&(id - 1))))
        .next()
        .unwrap_or(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    dbg!(part_one(&input));
    dbg!(part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(Seat::parse("FBFBBFFRLR"), Seat { row: 44, column: 5 });
        assert_eq!(Seat::parse("BFFFBBFRRR"), Seat { row: 70, column: 7 });
        assert_eq!(Seat::parse("FFFBBBFRRR"), Seat { row: 14, column: 7 });
        assert_eq!(
            Seat::parse("BBFFBBFRLL"),
            Seat {
                row: 102,
                column: 4
            }
        );
    }
}

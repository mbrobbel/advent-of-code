use std::{collections::HashSet, error::Error, io::Read, str::FromStr};

enum Direction {
    Right,
    Left,
    Down,
    Up,
}

struct Move {
    direction: Direction,
    distance: usize,
}

impl FromStr for Move {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let distance = s[1..].parse()?;
        let direction = match s.chars().nth(0).unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'U' => Direction::Up,
            _ => panic!(),
        };
        Ok(Move {
            direction,
            distance,
        })
    }
}

impl Move {
    fn to_steps(self) -> impl Iterator<Item = (isize, isize)> {
        std::iter::repeat(match self.direction {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        })
        .take(self.distance)
    }
}

#[derive(Clone, Debug)]
struct Circuit {
    steps: Vec<(isize, isize)>,
}

impl From<Circuit> for HashSet<(isize, isize)> {
    fn from(circuit: Circuit) -> HashSet<(isize, isize)> {
        circuit.steps.iter().cloned().collect()
    }
}

impl FromStr for Circuit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Circuit {
            steps: s
                .split(',')
                .map(Move::from_str)
                .map(Result::unwrap)
                .flat_map(Move::to_steps)
                .scan((0, 0), |position, step| {
                    position.0 += step.0;
                    position.1 += step.1;
                    Some(position.to_owned())
                })
                .collect(),
        })
    }
}

fn part_one(input: &str) -> usize {
    let wires: Vec<Circuit> = input
        .lines()
        .map(Circuit::from_str)
        .map(Result::unwrap)
        .collect();

    wires
        .iter()
        .cloned()
        .map(|x| HashSet::from(x))
        .fold(HashSet::new(), |mut intersections, set| {
            if intersections.is_empty() {
                intersections = set;
                intersections
            } else {
                intersections.intersection(&set).cloned().collect()
            }
        })
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap() as usize
}

fn part_two(input: &str) -> usize {
    let wires: Vec<Circuit> = input
        .lines()
        .map(Circuit::from_str)
        .map(Result::unwrap)
        .collect();

    wires
        .iter()
        .cloned()
        .map(|x| HashSet::from(x))
        .fold(HashSet::new(), |mut intersections, set| {
            if intersections.is_empty() {
                intersections = set;
                intersections
            } else {
                intersections.intersection(&set).cloned().collect()
            }
        })
        .iter()
        .map(|position| {
            wires[0].steps.iter().position(|x| x == position).unwrap()
                + wires[1].steps.iter().position(|x| x == position).unwrap()
        })
        .min()
        .unwrap() as usize
        + 2
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
        assert_eq!(part_one("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
        assert_eq!(
            part_one("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
        assert_eq!(
            part_one(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(
            part_two("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
        assert_eq!(
            part_two(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}

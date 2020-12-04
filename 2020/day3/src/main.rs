use std::io::Read;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl Tile {
    fn is_tree(&self) -> bool {
        self == &Tile::Tree
    }
}

impl From<char> for Tile {
    fn from(input: char) -> Self {
        match input {
            '#' => Tile::Tree,
            '.' => Tile::Open,
            _ => panic!("bad input"),
        }
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn traverse(&self, right: usize, down: usize) -> usize {
        (down..)
            .take_while(|&y| y < self.height)
            .step_by(down)
            .zip((right..).step_by(right).map(|x| x % self.width))
            .map(|(y, x)| self.tiles[x + y * self.width])
            .filter(Tile::is_tree)
            .count()
    }
}

fn parse_grid(input: &str) -> Grid {
    Grid {
        width: input.chars().position(|char| char == '\n').unwrap(),
        height: input.chars().filter(|&char| char == '\n').count(),
        tiles: input
            .chars()
            .filter(|&char| char != '\n')
            .map(Tile::from)
            .collect(),
    }
}

fn part_one(input: &str) -> usize {
    let grid = parse_grid(input);
    grid.traverse(3, 1)
}

fn part_two(input: &str) -> usize {
    let grid = parse_grid(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| grid.traverse(*right, *down))
        .fold(1, |acc, x| acc * x)
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

    static INPUT: &str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 7);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(INPUT), 336);
    }
}

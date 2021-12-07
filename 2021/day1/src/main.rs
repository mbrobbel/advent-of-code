use std::{fs, io::Error};

fn part_one(input: &[u16]) -> usize {
    input.windows(2).fold(0, |mut increments, pair| {
        if pair[1] > pair[0] {
            increments += 1;
        }
        increments
    })
}

fn part_two(input: &[u16]) -> usize {
    let windows = input
        .windows(3)
        .map(<[_]>::iter)
        .map(Iterator::sum)
        .collect::<Vec<_>>();
    part_one(&windows)
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input")?
        .lines()
        .filter_map(|reading| reading.parse().ok())
        .collect::<Vec<_>>();

    dbg!(part_one(&input));
    dbg!(part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [u16] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 7);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(INPUT), 5);
    }
}

use std::{collections::HashSet, io::Read};

fn find_two(input: impl AsRef<[usize]>) -> Option<usize> {
    let set: HashSet<usize> = input.as_ref().iter().copied().collect();
    input
        .as_ref()
        .iter()
        .find_map(|x| set.get(&(2020 - x)).map(|y| x * y))
}

fn find_three(input: impl AsRef<[usize]>) -> Option<usize> {
    for x in input.as_ref() {
        for y in input.as_ref() {
            for z in input.as_ref() {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let input = input
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    dbg!(find_two(&input));
    dbg!(find_three(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [usize; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn part_one() {
        assert_eq!(find_two(&INPUT), Some(514579));
    }

    #[test]
    fn part_two() {
        assert_eq!(find_three(&INPUT), Some(241861950));
    }
}

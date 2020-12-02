use std::io::Read;

#[derive(Debug, PartialEq)]
struct Entry<'input> {
    password: &'input str,
    policy: Policy,
}

impl Entry<'_> {
    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&char| self.policy.character == char)
            .count();
        count >= self.policy.start && count <= self.policy.end
    }

    fn is_valid2(&self) -> bool {
        self.password
            .chars()
            .nth(self.policy.start - 1)
            .filter(|&char| self.policy.character == char)
            .xor(
                self.password
                    .chars()
                    .nth(self.policy.end - 1)
                    .filter(|&char| self.policy.character == char),
            )
            .is_some()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Policy {
    start: usize,
    end: usize,
    character: char,
}

impl<'input> From<&'input str> for Entry<'input> {
    fn from(input: &'input str) -> Entry<'input> {
        let split = input.find('-').unwrap();
        let ws = input.find(' ').unwrap();
        let colon = input.find(':').unwrap();

        Entry {
            password: &input[colon + 2..],
            policy: Policy {
                start: input[..split].parse().unwrap(),
                end: input[split + 1..ws].parse().unwrap(),
                character: input.chars().nth(colon - 1).unwrap(),
            },
        }
    }
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(Entry::from)
        .filter(Entry::is_valid)
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(Entry::from)
        .filter(Entry::is_valid2)
        .count()
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

    static INPUT: &str = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn parse() {
        assert_eq!(
            Entry::from(INPUT.lines().next().unwrap()),
            Entry {
                password: "abcde",
                policy: Policy {
                    start: 1,
                    end: 3,
                    character: 'a'
                }
            }
        );
    }

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 2);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(INPUT), 1);
    }
}

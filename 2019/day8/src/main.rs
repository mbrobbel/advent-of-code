use std::{collections::HashMap, error::Error, fmt, io::Read};

fn part_one(input: &str, width: usize, height: usize) -> usize {
    input
        .chars()
        .flat_map(|x| x.to_digit(10))
        .collect::<Vec<u32>>()[..]
        .chunks(width * height)
        .fold(vec![], |mut maps, layer| {
            let mut map = HashMap::new();
            for digit in layer {
                map.entry(*digit)
                    .and_modify(|e| {
                        *e += 1;
                    })
                    .or_insert(1);
            }
            maps.push(map);
            maps
        })
        .iter()
        .min_by_key(|x| x.get(&0))
        .map(|x| x.get(&1).unwrap_or(&0) * x.get(&2).unwrap_or(&0))
        .unwrap_or(0) as usize
}

fn part_two(input: &str, width: usize, height: usize) -> String {
    Image(
        input
            .chars()
            .flat_map(|x| x.to_digit(10))
            .collect::<Vec<u32>>()[..]
            .chunks(width * height)
            .fold(vec![], |mut image, layer| {
                image.push(layer.to_vec());
                image
            }),
        width,
        height,
    )
    .to_string()
}

type Layer = Vec<u32>;
struct Image(Vec<Layer>, usize, usize);

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.2 {
            for x in 0..self.1 {
                write!(
                    f,
                    "{}",
                    self.0
                        .iter()
                        .map(|layer| layer[y * self.1 + x])
                        .find(|p| *p != 2)
                        .map(|p| match p {
                            0 => "■",
                            1 => "□",
                            _ => unreachable!(),
                        })
                        .unwrap_or(" ")
                )?;
                if x == self.1 - 1 {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("{}", part_one(&input, 25, 6));
    println!("{}", part_two(&input, 25, 6));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(part_one("123456789012", 3, 2), 1);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two("0222112222120000", 2, 2), "■□\n□■\n");
    }
}

use aoc_2019_intcode::Intcode;
use std::{error::Error, io::Read, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!(
        "part 1: {:?}",
        Intcode::from_str(&input).unwrap().run(vec![1]).output
    );
    println!(
        "part 2: {:?}",
        Intcode::from_str(&input).unwrap().run(vec![2]).output
    );

    Ok(())
}

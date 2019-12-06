use aoc_2019_intcode::Intcode;
use std::{error::Error, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let program: Vec<isize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<isize>().ok())
        .collect();

    println!("part 1");
    Intcode::with_capacity(program.len())
        .load(program.clone())
        .run(1);

    println!("part 2");
    Intcode::with_capacity(program.len()).load(program).run(5);

    Ok(())
}

use logos::Logos;
use std::{collections::HashMap, io::Read};

#[derive(Debug, Logos)]
enum Token<'a> {
    #[error]
    #[token("bag", logos::skip)]
    #[token("bags", logos::skip)]
    #[token("bags contain", logos::skip)]
    #[token("no other bags.", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    #[regex("[A-z]+ [A-z]+", |lex| lex.slice())]
    Text(&'a str),

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(usize),

    #[token(",")]
    Comma,
}

type Rules<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

fn rules(input: &str) -> Rules {
    input
        .lines()
        .map(|input| {
            let mut lex = Token::lexer(input);
            let name = match lex.next() {
                Some(Token::Text(color)) => color,
                _ => panic!("bad input"),
            };
            let mut contains = Vec::default();
            loop {
                match lex.next() {
                    Some(Token::Number(count)) => {
                        if let Some(Token::Text(color)) = lex.next() {
                            contains.push((color, count));
                        } else {
                            panic!("bad input")
                        }
                    }
                    Some(Token::Comma) => {}
                    _ => break,
                }
            }
            (name, contains)
        })
        .collect()
}

fn contains_gold(bag: &str, rules: &Rules) -> bool {
    bag == "shiny gold"
        || rules[bag]
            .iter()
            .fold(false, |acc, (x, _)| acc | contains_gold(x, rules))
}

fn contains_count(bag: &str, rules: &Rules) -> usize {
    rules[bag]
        .iter()
        .fold(0, |acc, (x, y)| acc + y + y * contains_count(x, rules))
}

fn part_one(input: &str) -> usize {
    let rules = rules(input);
    rules
        .keys()
        .filter(|&bag| *bag != "shiny gold")
        .filter(|bag| contains_gold(bag, &rules))
        .count()
}

fn part_two(input: &str) -> usize {
    let rules = rules(input);
    contains_count("shiny gold", &rules)
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

    static INPUT: &str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn one() {
        let rules = rules(INPUT);
        assert_eq!(rules.len(), 9);
        assert_eq!(contains_gold("light red", &rules), true);
        assert_eq!(contains_gold("dark orange", &rules), true);
        assert_eq!(contains_gold("muted yellow", &rules), true);
        assert_eq!(contains_gold("shiny gold", &rules), true);
        assert_eq!(contains_gold("dark olive", &rules), false);
        assert_eq!(contains_gold("vibrant plum", &rules), false);
        assert_eq!(contains_gold("faded blue", &rules), false);
        assert_eq!(contains_gold("dotted black", &rules), false);

        assert_eq!(part_one(INPUT), 4);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(INPUT), 32);
    }
}

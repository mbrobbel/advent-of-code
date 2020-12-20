use logos::Logos;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
    iter::FromIterator,
};

#[derive(Debug)]
struct ParseError;

trait Parse: Sized {
    fn parse(input: &str) -> Result<Self, ParseError>;
}

#[derive(Logos, Debug, PartialEq)]
enum Token<'input> {
    #[token("\n\n")]
    Break,
    #[token(":")]
    Colon,
    #[token("#")]
    Hash,
    #[regex("[a-zA-Z]+", |lex| lex.slice())]
    Text(&'input str),
    #[regex("#[0-9a-f]+", |lex| &lex.slice()[1..])]
    Color(&'input str),
    #[regex("[0-9]+")]
    Number,
    #[error]
    #[token("\n", logos::skip)]
    #[token(" ", logos::skip)]
    Error,
}

#[derive(Debug, PartialEq)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}
impl Parse for EyeColor {
    fn parse(input: &str) -> Result<Self, ParseError> {
        match input {
            "amb" => Ok(EyeColor::Amb),
            "blu" => Ok(EyeColor::Blu),
            "brn" => Ok(EyeColor::Brn),
            "gry" => Ok(EyeColor::Gry),
            "grn" => Ok(EyeColor::Grn),
            "hzl" => Ok(EyeColor::Hzl),
            "oth" => Ok(EyeColor::Oth),
            _ => Err(ParseError),
        }
    }
}

struct BirthYear(u16);
impl Parse for BirthYear {
    fn parse(input: &str) -> Result<Self, ParseError> {
        match input.parse::<u16>() {
            Ok(x) if x >= 1920 && x <= 2002 => Ok(Self(x)),
            _ => Err(ParseError),
        }
    }
}

struct IssueYear(u16);
impl Parse for IssueYear {
    fn parse(input: &str) -> Result<Self, ParseError> {
        match input.parse::<u16>() {
            Ok(x) if x >= 2010 && x <= 2020 => Ok(Self(x)),
            _ => Err(ParseError),
        }
    }
}

struct ExpirationYear(u16);
impl Parse for ExpirationYear {
    fn parse(input: &str) -> Result<Self, ParseError> {
        match input.parse::<u16>() {
            Ok(x) if x >= 2020 && x <= 2030 => Ok(Self(x)),
            _ => Err(ParseError),
        }
    }
}

enum Unit {
    Cm,
    Inch,
}
impl Parse for Unit {
    fn parse(input: &str) -> Result<Self, ParseError> {
        match input {
            "cm" => Ok(Unit::Cm),
            "in" => Ok(Unit::Inch),
            _ => Err(ParseError),
        }
    }
}

struct Height {
    pub height: u8,
    pub unit: Unit,
}
impl Parse for Height {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let mut lex = Token::lexer(input);
        if let Some(Token::Number) = lex.next() {
            let height = lex.slice().parse::<u8>().map_err(|_| ParseError)?;
            if let Some(Token::Text(input)) = lex.next() {
                let unit = match Unit::parse(input) {
                    Ok(Unit::Cm) if height >= 150 && height <= 193 => Ok(Unit::Cm),
                    Ok(Unit::Inch) if height >= 59 && height <= 76 => Ok(Unit::Inch),
                    _ => Err(ParseError),
                }?;
                return Ok(Self {
                    height: height as u8,
                    unit,
                });
            }
        };
        Err(ParseError)
    }
}

pub struct HairColor(u32);
impl Parse for HairColor {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let mut lex = Token::lexer(input);
        if let Some(Token::Color(color)) = lex.next() {
            if color.len() == 6 {
                Ok(Self(
                    u32::from_str_radix(color, 16).map_err(|_| ParseError)?,
                ))
            } else {
                Err(ParseError)
            }
        } else {
            Err(ParseError)
        }
    }
}

pub struct PassportID(u32);
impl Parse for PassportID {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let mut lex = Token::lexer(input);
        if let Some(Token::Number) = lex.next() {
            let number = lex.slice();
            if number.len() == 9 {
                Ok(PassportID(
                    u32::from_str_radix(number, 10).map_err(|_| ParseError)?,
                ))
            } else {
                Err(ParseError)
            }
        } else {
            Err(ParseError)
        }
    }
}

struct Passport {
    pub byr: BirthYear,
    pub iyr: IssueYear,
    pub eyr: ExpirationYear,
    pub hgt: Height,
    pub hcl: HairColor,
    pub ecl: EyeColor,
    pub pid: PassportID,
}

impl Parse for Passport {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let map = passport_as_map(input);
        let get = |key: &str| -> Result<_, ParseError> { map.get(key).ok_or(ParseError) };
        Ok(Self {
            byr: get("byr").and_then(|&x| BirthYear::parse(x))?,
            iyr: get("iyr").and_then(|&x| IssueYear::parse(x))?,
            eyr: get("eyr").and_then(|&x| ExpirationYear::parse(x))?,
            hgt: get("hgt").and_then(|&x| Height::parse(x))?,
            hcl: get("hcl").and_then(|&x| HairColor::parse(x))?,
            ecl: get("ecl").and_then(|&x| EyeColor::parse(x))?,
            pid: get("pid").and_then(|&x| PassportID::parse(x))?,
        })
    }
}

fn required_fields() -> HashSet<&'static str> {
    HashSet::from_iter(
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .copied(),
    )
}

fn passport_as_map(passport: &str) -> HashMap<&str, &str> {
    passport
        .split(char::is_whitespace)
        .filter(|x| x.len() > 1)
        .map(|x| {
            let mut split = x.split(":");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|&passport| {
            let map = passport_as_map(passport);
            HashSet::from_iter(map.keys().copied())
                .intersection(&required_fields())
                .count()
                == 7
        })
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|input| Passport::parse(input))
        .filter(Result::is_ok)
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

    static INPUT_1: &str = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

    static INPUT_2: &str = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

    static INPUT_3: &str = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT_1), 2);
    }

    #[test]
    fn two() {
        assert!(BirthYear::parse("2002").is_ok());
        assert!(BirthYear::parse("2002").is_ok());
        assert!(BirthYear::parse("2003").is_err());

        assert!(Height::parse("60in").is_ok());
        assert!(Height::parse("190cm").is_ok());
        assert!(Height::parse("190in").is_err());
        assert!(Height::parse("190").is_err());

        assert!(HairColor::parse("#123abc").is_ok());
        assert!(HairColor::parse("#123abz").is_err());
        assert!(HairColor::parse("123abc").is_err());

        assert!(EyeColor::parse("brn").is_ok());
        assert!(EyeColor::parse("wat").is_err());

        assert!(PassportID::parse("000000001").is_ok());
        assert!(PassportID::parse("0123456789").is_err());

        assert_eq!(part_two(INPUT_2), 0);
        assert_eq!(part_two(INPUT_3), 4);
    }
}

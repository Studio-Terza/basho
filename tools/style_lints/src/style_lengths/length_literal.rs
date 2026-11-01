use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

pub struct LengthLiteral {
    number: String,
    unit: String,
}

impl LengthLiteral {
    const UNITS: [&str; 3] = ["px", "ms", "s"];
    const ZERO: &str = "0";

    fn all_in(characters: &[char]) -> Vec<LengthLiteral> {
        characters
            .iter()
            .enumerate()
            .filter(|(at, character)| {
                character.is_ascii_digit()
                    && at
                        .checked_sub(1)
                        .and_then(|before| characters.get(before))
                        .is_none_or(|before| {
                            !before.is_ascii_alphanumeric() && *before != '.' && *before != '#'
                        })
            })
            .filter_map(|(at, _)| Self::starting(&characters[at..]))
            .collect()
    }

    fn starting(characters: &[char]) -> Option<LengthLiteral> {
        let number: String = characters
            .iter()
            .take_while(|character| character.is_ascii_digit() || **character == '.')
            .collect();
        let unit: String = characters
            .iter()
            .skip(number.len())
            .take_while(|character| character.is_ascii_alphabetic())
            .collect();
        (Self::UNITS.contains(&unit.as_str()) && number.trim_end_matches('.') != Self::ZERO)
            .then_some(LengthLiteral { number, unit })
    }
}

impl fmt::Display for LengthLiteral {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.number, self.unit)
    }
}

#[derive(newtype::Newtype)]
pub struct LengthLiterals(Vec<LengthLiteral>);

impl FromStr for LengthLiterals {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<LengthLiterals, Infallible> {
        let characters: Vec<char> = line.chars().collect();
        Ok(LengthLiterals(LengthLiteral::all_in(&characters)))
    }
}

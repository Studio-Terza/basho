use std::convert::Infallible;
use std::str::FromStr;

#[derive(derive_more::Deref)]
#[deref(forward)]
pub struct Uncommented(String);

impl Uncommented {
    const OPEN: &str = "/*";
    const CLOSE: &str = "*/";

    fn blank(character: char) -> char {
        if character == '\n' { '\n' } else { ' ' }
    }

    fn cleared(text: &str) -> String {
        text.split_once(Self::OPEN).map_or_else(
            || text.to_owned(),
            |(head, tail)| {
                let (comment, rest) = tail.split_once(Self::CLOSE).unwrap_or((tail, ""));
                let blank: String = format!("{}{comment}{}", Self::OPEN, Self::CLOSE)
                    .chars()
                    .map(Self::blank)
                    .collect();
                head.to_owned() + &blank + &Self::cleared(rest)
            },
        )
    }
}

impl FromStr for Uncommented {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Uncommented, Infallible> {
        Ok(Uncommented(Self::cleared(text)))
    }
}

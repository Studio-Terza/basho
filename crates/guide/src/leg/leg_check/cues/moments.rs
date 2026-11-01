use std::convert::Infallible;
use std::str::FromStr;

use crate::clock::ClockTime;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Moments {
    times: Vec<ClockTime>,
}

impl Moments {
    const MINUTE_LENGTH: usize = 2;
    const SERVICE: &str = "the ";

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.times.is_empty()
    }
}

impl FromStr for Moments {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Moments, Infallible> {
        let times = text
            .match_indices(':')
            .filter_map(|(colon, _)| {
                let before = text.get(..colon)?;
                let hour = before
                    .chars()
                    .rev()
                    .take_while(char::is_ascii_digit)
                    .count();
                let start = colon.checked_sub(hour).filter(|_| hour > 0)?;
                let written = text.get(start..=colon + Self::MINUTE_LENGTH)?;
                let prefix = before.get(..start)?;
                ClockTime::try_from(written.to_owned())
                    .ok()
                    .filter(|_| !prefix.ends_with(Self::SERVICE))
            })
            .collect();
        Ok(Moments { times })
    }
}

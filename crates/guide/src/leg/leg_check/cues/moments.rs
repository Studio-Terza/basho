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
    pub fn first(&self) -> Option<ClockTime> {
        self.times.first().copied()
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
                let time = ClockTime::try_from(written.to_owned()).ok();
                time.xor(prefix.strip_suffix(Self::SERVICE).and(time))
            })
            .collect();
        Ok(Moments { times })
    }
}

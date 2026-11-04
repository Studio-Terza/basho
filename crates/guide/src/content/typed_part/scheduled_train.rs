use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use strum::{Display, EnumString};

use crate::content::{ContentError, Figure};
use crate::words::Written;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
enum TrainBrand {
    Hayabusa,
    Yamabiko,
    Komachi,
    Tsubasa,
    Inaho,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct ScheduledTrain {
    brand: TrainBrand,
    number: Figure,
}

impl FromStr for ScheduledTrain {
    type Err = ContentError;

    fn from_str(text: &str) -> Result<ScheduledTrain, ContentError> {
        text.split_once(' ')
            .and_then(|(brand, number)| {
                Some(ScheduledTrain {
                    brand: brand.parse().ok()?,
                    number: Figure::try_from(number).ok()?,
                })
            })
            .ok_or_else(|| ContentError::BadTrain(Written::new(text.to_owned())))
    }
}

impl TryFrom<String> for ScheduledTrain {
    type Error = ContentError;

    fn try_from(text: String) -> Result<ScheduledTrain, ContentError> {
        text.parse()
    }
}

impl fmt::Display for ScheduledTrain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.brand, self.number)
    }
}

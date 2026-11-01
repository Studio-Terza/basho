use std::fmt;

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

impl ScheduledTrain {
    fn read(text: &str) -> Option<ScheduledTrain> {
        let (brand, number) = text.split_once(' ')?;
        Some(ScheduledTrain {
            brand: brand.parse().ok()?,
            number: Figure::try_from(number).ok()?,
        })
    }
}

impl TryFrom<String> for ScheduledTrain {
    type Error = ContentError;

    fn try_from(text: String) -> Result<ScheduledTrain, ContentError> {
        Self::read(&text).ok_or(ContentError::BadTrain(Written::new(text)))
    }
}

impl fmt::Display for ScheduledTrain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.brand, self.number)
    }
}

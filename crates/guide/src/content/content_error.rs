use crate::words::Written;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("a clock time sits in the words {0:?}; write it as a time part")]
    ClockInWords(Written),
    #[error("a distance sits in the words {0:?}; write it as a distance part")]
    DistanceInWords(Written),
    #[error("{0}")]
    Unread(Written),
    #[error("{0:?} is not a clock time")]
    BadClock(Written),
    #[error("{0:?} is not an amount with its unit")]
    BadMeasure(Written),
    #[error("{0:?} is not a price in yen")]
    BadPrice(Written),
    #[error("{0:?} is not a named train with its number")]
    BadTrain(Written),
}

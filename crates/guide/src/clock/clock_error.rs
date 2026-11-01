use crate::words::Written;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClockError {
    #[error("{0:?} is not a date written YYYY-MM-DD")]
    Date(Written),
    #[error("{0:?} is not a time written HH:MM")]
    Time(Written),
    #[error("{0:?} is not a UTC offset written +HH:MM")]
    Zone(Written),
}

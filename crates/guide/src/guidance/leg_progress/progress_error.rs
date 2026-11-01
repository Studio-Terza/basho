use crate::words::Written;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProgressError {
    #[error("kept progress {0:?} isn't `day N leg N behind N`")]
    Unreadable(Written),
}

use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unreadable {
    Lines,
    Progress,
    Stamp,
}

impl fmt::Display for Unreadable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unreadable::Lines => write!(formatter, "the progress file does not have two lines"),
            Unreadable::Progress => write!(formatter, "the progress line is unreadable"),
            Unreadable::Stamp => write!(formatter, "the legs stamp is unreadable"),
        }
    }
}

impl Error for Unreadable {}

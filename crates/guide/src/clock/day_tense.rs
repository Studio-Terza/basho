use crate::clock::ClockTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayTense {
    Ahead,
    Today,
    Past,
}

impl DayTense {
    #[must_use]
    pub fn today<T>(self, found: Option<T>) -> Option<T> {
        match self {
            DayTense::Today => found,
            DayTense::Ahead | DayTense::Past => None,
        }
    }

    #[must_use]
    pub fn leaving(self, early: Option<ClockTime>) -> DayTense {
        match self {
            DayTense::Today => Self::held(early),
            DayTense::Ahead | DayTense::Past => self,
        }
    }

    fn held(early: Option<ClockTime>) -> DayTense {
        match early {
            Some(_) => DayTense::Ahead,
            None => DayTense::Today,
        }
    }
}

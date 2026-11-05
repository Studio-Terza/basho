use crate::geo::Distance;
use crate::guidance::nearness::Nearness;
use crate::leg::Leg;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    Arrived,
    Underway,
}

impl From<Ordering> for Reach {
    fn from(against_gate: Ordering) -> Reach {
        match against_gate {
            Ordering::Less => Reach::Underway,
            Ordering::Equal | Ordering::Greater => Reach::Arrived,
        }
    }
}

impl Reach {
    #[must_use]
    pub fn finishing(next: Option<&Leg>, here: Option<&Leg>, along: Distance) -> Reach {
        match next {
            Some(_) => Reach::Underway,
            None => Reach::at_end(here, along),
        }
    }

    fn at_end(here: Option<&Leg>, along: Distance) -> Reach {
        match here {
            Some(leg) => Reach::from(along.cmp(&(leg.length() - Nearness::GpsError.distance()))),
            None => Reach::Underway,
        }
    }
}

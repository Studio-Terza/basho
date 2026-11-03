use guide::day_number::DayNumber;
use guide::leg::LegIndex;
use guide::words::{Slug, StarName};

#[derive(newtype::Newtype, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StopIndex(usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EntryKey {
    Index,
    Shops,
    Day(DayNumber),
    Goal {
        day: DayNumber,
        first: LegIndex,
    },
    Step {
        day: DayNumber,
        leg: LegIndex,
    },
    Stop {
        day: DayNumber,
        leg: LegIndex,
        stop: StopIndex,
    },
    Star {
        day: DayNumber,
        short: StarName,
    },
    Place(Slug),
    Inn(DayNumber),
    Evening(DayNumber),
}

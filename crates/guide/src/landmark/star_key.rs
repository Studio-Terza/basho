use crate::StarPoint;
use crate::day_number::DayNumber;
use crate::words::StarName;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StarKey<'name> {
    day: DayNumber,
    name: &'name StarName,
}

impl<'name> StarKey<'name> {
    #[must_use]
    pub fn new(day: DayNumber, name: &'name StarName) -> StarKey<'name> {
        StarKey { day, name }
    }

    #[must_use]
    pub fn day(self) -> DayNumber {
        self.day
    }

    #[must_use]
    pub fn name(self) -> &'name StarName {
        self.name
    }
}

impl<'name> From<&'name StarPoint> for StarKey<'name> {
    fn from(point: &'name StarPoint) -> StarKey<'name> {
        StarKey::new(DayNumber::from(point), point.short())
    }
}

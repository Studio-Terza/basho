use crate::guidance::Guidance;
use crate::leg::{Leg, LegIndex, Mode};
use crate::times::{ListedTime, ShownTime};
use std::cmp::Ordering;

impl Guidance<'_> {
    #[must_use]
    pub fn shifted(&self, leg: LegIndex, planned: ListedTime) -> ShownTime {
        match leg.cmp(&self.leg) {
            Ordering::Less => ShownTime::from(planned),
            Ordering::Equal | Ordering::Greater => {
                self.moved(self.day.leg(leg).map(Leg::mode), leg, planned)
            }
        }
    }

    fn moved(&self, mode: Option<Mode>, leg: LegIndex, planned: ListedTime) -> ShownTime {
        match mode {
            Some(Mode::Walk) => self.timeline().holding(leg).shown(planned, self.offset),
            Some(Mode::Train | Mode::Bus | Mode::Boat | Mode::Car) | None => {
                ShownTime::from(planned)
            }
        }
    }
}

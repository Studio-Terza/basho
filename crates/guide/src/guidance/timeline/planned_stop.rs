use std::cmp::Ordering;
use std::ops::Range;

use crate::clock::{ClockTime, Minutes};
use crate::geo::{Coordinate, Distance};
use crate::leg::LegIndex;
use crate::ruby_text::RubyText;

#[derive(derive_more::AsRef, Clone, Copy, Debug)]
pub struct PlannedStop<'day> {
    leg: LegIndex,
    along: Distance,
    reach: Minutes,
    planned: ClockTime,
    booked: Minutes,
    name: &'day RubyText,
    #[as_ref]
    at: Coordinate,
}

impl<'day> PlannedStop<'day> {
    #[must_use]
    pub fn new(leg: LegIndex, booked: Minutes, name: &'day RubyText, at: Coordinate) -> Self {
        PlannedStop {
            leg,
            along: Distance::default(),
            reach: Minutes::ZERO,
            planned: ClockTime::at_hour(0),
            booked,
            name,
            at,
        }
    }

    #[must_use]
    pub fn reached_at(self, reach: Minutes) -> PlannedStop<'day> {
        PlannedStop { reach, ..self }
    }

    #[must_use]
    pub fn placed_at(self, along: Distance, planned: ClockTime) -> PlannedStop<'day> {
        PlannedStop {
            along,
            planned,
            ..self
        }
    }

    #[must_use]
    pub fn leg(&self) -> LegIndex {
        self.leg
    }

    #[must_use]
    pub fn along(&self) -> Distance {
        self.along
    }

    #[must_use]
    pub fn visit(&self) -> Range<Minutes> {
        self.reach..self.reach + self.booked
    }

    #[must_use]
    pub fn planned(&self) -> ClockTime {
        self.planned
    }

    #[must_use]
    pub fn booked(&self) -> Minutes {
        self.booked
    }

    #[must_use]
    pub fn name(&self) -> &'day RubyText {
        self.name
    }

    #[must_use]
    pub fn settled(&self, time: Minutes, spent: Option<Minutes>, here: Minutes) -> Minutes {
        let inferred = time - self.planned.since_midnight();
        let spent = spent.unwrap_or(inferred).clamp(Minutes::ZERO, self.booked);
        match spent.cmp(&Minutes::ZERO) {
            Ordering::Equal => here.min(self.reach),
            Ordering::Less | Ordering::Greater => self.stayed(spent, here),
        }
    }

    fn stayed(&self, spent: Minutes, here: Minutes) -> Minutes {
        match spent.cmp(&self.booked) {
            Ordering::Equal => here.max(self.reach + spent),
            Ordering::Less | Ordering::Greater => self.reach + spent,
        }
    }
}

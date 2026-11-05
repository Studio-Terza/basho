use crate::clock::{ClockTime, Minutes};
use crate::deadline_kind::StretchEnd;
use crate::guidance::timeline::deadline::Deadline;
use crate::guidance::timeline::planned_stop::PlannedStop;
use crate::guidance::timeline::stretch_index::StretchIndex;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lead {
    Ahead,
    Behind,
}

impl From<Ordering> for Lead {
    fn from(against_event: Ordering) -> Lead {
        match against_event {
            Ordering::Less | Ordering::Equal => Lead::Ahead,
            Ordering::Greater => Lead::Behind,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Here {
    stretch: StretchIndex,
    reach: Minutes,
    time: Minutes,
    clock: Option<Minutes>,
}

impl Here {
    #[must_use]
    pub fn new(
        stretch: StretchIndex,
        reach: Minutes,
        time: Minutes,
        clock: Option<Minutes>,
    ) -> Here {
        Here {
            stretch,
            reach,
            time,
            clock,
        }
    }

    #[must_use]
    pub fn clock(self) -> Option<Minutes> {
        self.clock
    }

    #[must_use]
    pub fn stretch(self) -> StretchIndex {
        self.stretch
    }

    #[must_use]
    pub fn projected(self, reach: Minutes) -> Minutes {
        self.time + reach - self.reach
    }

    #[must_use]
    pub fn settled(
        self,
        stop: Option<&PlannedStop<'_>>,
        spent: Option<Minutes>,
        waiting: Option<ClockTime>,
        arrived: Option<ClockTime>,
    ) -> Here {
        let early = waiting.map(|start| (self.time - start.since_midnight()).min(Minutes::ZERO));
        let there =
            arrived.map(|planned| (self.time - planned.since_midnight()).max(Minutes::ZERO));
        let settled = stop.map_or_else(
            || self.reach + early.unwrap_or(Minutes::ZERO),
            |stop| stop.settled(self.time, spent, self.reach),
        );
        let reach = there
            .filter(|late| *late > Minutes::ZERO)
            .map_or(settled, |late| settled.max(self.reach + late));
        Here { reach, ..self }
    }

    #[must_use]
    pub fn capped(self, most: Option<Minutes>) -> Here {
        Here {
            reach: most.map_or(self.reach, |most| self.reach.min(most)),
            ..self
        }
    }

    #[must_use]
    pub fn ahead_of(self, deadline: &Deadline<'_>) -> Lead {
        match deadline.kind().ending() {
            StretchEnd::Ends => Lead::Ahead,
            StretchEnd::Continues => self.by_reach(deadline),
        }
    }

    fn by_reach(self, deadline: &Deadline<'_>) -> Lead {
        match deadline.reach().cmp(&self.reach) {
            Ordering::Less => Lead::Behind,
            Ordering::Equal | Ordering::Greater => self.by_clock(deadline),
        }
    }

    fn by_clock(self, deadline: &Deadline<'_>) -> Lead {
        match self.clock {
            None => Lead::Ahead,
            Some(clock) => Lead::from(clock.cmp(&deadline.event().since_midnight())),
        }
    }
}

use guide::itinerary::DayNumber;
use guide::leg::LegIndex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Home {
    Day(DayNumber),
    Goal { day: DayNumber, first: LegIndex },
    Step { day: DayNumber, leg: LegIndex },
}

impl Home {
    #[must_use]
    pub fn home_day(self) -> DayNumber {
        match self {
            Home::Day(day) | Home::Goal { day, .. } | Home::Step { day, .. } => day,
        }
    }

    pub fn reached<Out>(
        self,
        day: Out,
        goal: impl FnOnce(LegIndex) -> Out,
        step: impl FnOnce(LegIndex) -> Out,
    ) -> Out {
        match self {
            Home::Day(_) => day,
            Home::Goal { first, .. } => goal(first),
            Home::Step { leg, .. } => step(leg),
        }
    }
}

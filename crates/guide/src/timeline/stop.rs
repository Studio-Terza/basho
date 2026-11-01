use crate::clock::Minutes;
use crate::geo::Distance;
use crate::timeline::planned_stop::PlannedStop;

#[derive(Clone, Copy, Debug)]
pub struct Stop<'day> {
    planned: PlannedStop<'day>,
    spent: Minutes,
}

impl<'day> Stop<'day> {
    pub const REACH: Distance = Distance::new(80.0);
    pub const AT: Distance = Distance::new(30.0);

    #[must_use]
    pub fn new(planned: PlannedStop<'day>, spent: Minutes) -> Stop<'day> {
        Stop { planned, spent }
    }

    #[must_use]
    pub fn planned(&self) -> PlannedStop<'day> {
        self.planned
    }

    #[must_use]
    pub fn spent(&self) -> Minutes {
        self.spent
    }
}

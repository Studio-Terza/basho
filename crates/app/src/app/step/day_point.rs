use std::cmp::Ordering;

use guide::itinerary::{Day, Itinerary};

use guide::guidance::{Guidance, Phase};

use super::Step;
use super::spot::Spot;
use super::step_number::StepNumber;
use super::step_part::StepPart;
use super::step_state::StepState;
use super::toward::Toward;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayPoint {
    Ahead,
    At(StepNumber),
    Finished,
}

impl DayPoint {
    #[must_use]
    pub fn locate(guidance: &Guidance<'_>, steps: &[Step<'_>]) -> DayPoint {
        let part = guidance.leg().map_or(StepPart::Walk, |leg| {
            StepPart::during(leg.mode(), guidance.phase())
        });
        steps
            .iter()
            .filter(|_| guidance.phase() != Phase::Done)
            .find(|step| step.index() == guidance.leg_index() && step.part() == part)
            .map_or(DayPoint::Finished, |step| DayPoint::At(step.number()))
    }

    #[must_use]
    pub fn seen_from(self, day: &Day, today: Option<&Day>) -> DayPoint {
        match today.map(|today| today.date().cmp(&day.date())) {
            Some(Ordering::Equal) | None => self,
            Some(Ordering::Greater) => DayPoint::Finished,
            Some(Ordering::Less) => DayPoint::Ahead,
        }
    }

    #[must_use]
    pub fn state_of(self, step: StepNumber) -> StepState {
        match self {
            DayPoint::Finished => StepState::Done,
            DayPoint::At(current) if step < current => StepState::Done,
            DayPoint::At(current) if step == current => StepState::Current,
            DayPoint::At(_) | DayPoint::Ahead => StepState::Later,
        }
    }

    #[must_use]
    pub fn current(self) -> Option<StepNumber> {
        match self {
            DayPoint::At(current) => Some(current),
            DayPoint::Ahead | DayPoint::Finished => None,
        }
    }

    #[must_use]
    pub fn stepped(self, day: &Day, toward: Toward, itinerary: &Itinerary) -> Option<Spot> {
        match (self, toward) {
            (DayPoint::At(step), _) => Spot::new(day.number(), step).stepped(toward, itinerary),
            (DayPoint::Ahead, Toward::Next) => Some(Spot::entering(day, Toward::Next)),
            (DayPoint::Finished, Toward::Previous) => Some(Spot::entering(day, Toward::Previous)),
            (DayPoint::Ahead, Toward::Previous) => {
                Spot::entering(day, Toward::Next).stepped(toward, itinerary)
            }
            (DayPoint::Finished, Toward::Next) => {
                Spot::entering(day, Toward::Previous).stepped(toward, itinerary)
            }
        }
    }
}

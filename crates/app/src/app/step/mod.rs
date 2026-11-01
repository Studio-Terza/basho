mod day_point;
mod goal_number;
mod page_anchor;
mod spot;
mod step_number;
mod step_part;
mod step_state;
mod toward;

use guide::clock::Minutes;
use guide::guidance::tracking::{Placing, Stage};
use guide::times::ListedTime;

use guide::itinerary::Day;

use guide::end::End;

use guide::leg::{Leg, LegIndex};

pub use day_point::DayPoint;
pub use goal_number::GoalNumber;
pub use page_anchor::PageAnchor;
pub use spot::Spot;
pub use step_number::StepNumber;
pub use step_part::StepPart;
pub use step_state::StepState;
pub use toward::Toward;

#[derive(Clone, Copy, Debug)]
pub struct Step<'day> {
    number: StepNumber,
    index: LegIndex,
    leg: &'day Leg,
    part: StepPart,
}

impl<'day> Step<'day> {
    #[must_use]
    pub fn of_day(day: &'day Day) -> Vec<Step<'day>> {
        day.legs()
            .iter()
            .enumerate()
            .flat_map(|(position, leg)| {
                StepPart::for_mode(leg.mode()).iter().map(move |part| Step {
                    number: StepNumber::default(),
                    index: LegIndex::new(position),
                    leg,
                    part: *part,
                })
            })
            .enumerate()
            .map(|(position, step)| Step {
                number: StepNumber::from_position(position),
                ..step
            })
            .collect()
    }

    #[must_use]
    pub fn number(&self) -> StepNumber {
        self.number
    }

    #[must_use]
    pub fn index(&self) -> LegIndex {
        self.index
    }

    #[must_use]
    pub fn leg(&self) -> &'day Leg {
        self.leg
    }

    #[must_use]
    pub fn part(&self) -> StepPart {
        self.part
    }

    #[must_use]
    pub fn placing(&self) -> Placing {
        Placing::new(
            self.index,
            match self.part {
                StepPart::Board => Stage::Boarding,
                StepPart::Walk | StepPart::Alight => Stage::Under,
            },
        )
    }

    #[must_use]
    pub fn finished_in(&self, steps: &[Step<'_>]) -> Placing {
        steps
            .iter()
            .find(|later| later.number > self.number)
            .map_or_else(|| Placing::new(self.index, Stage::Finished), Step::placing)
    }

    #[must_use]
    pub fn due(&self) -> ListedTime {
        match self.part {
            StepPart::Board => self.leg.listed(End::Start),
            StepPart::Walk | StepPart::Alight => self.leg.listed(End::Finish),
        }
    }

    #[must_use]
    pub fn planned(&self) -> Minutes {
        match self.part {
            StepPart::Board => Minutes::ZERO,
            StepPart::Walk | StepPart::Alight => self.leg.minutes(),
        }
    }
}

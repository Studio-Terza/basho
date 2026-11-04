mod ending;
use crate::content::Content;
use crate::day_number::DayNumber;
use crate::end::End;
use crate::geo::Coordinate;
use crate::geo::Distance;
use crate::itinerary::day::goal::{Aim, Goal, LoopCheck, Purpose};
use crate::itinerary::day::{Caption, Night};
use crate::itinerary::{Finding, Problem};
use crate::leg::{Leg, LegIndex, Mode};
use crate::leg_line::LegLine;
use crate::words::Label;
pub use ending::Ending;
use newtype::Counter;

pub struct GoalPlan<'day> {
    day: DayNumber,
    legs: &'day [Leg],
    aims: &'day [Aim],
    night: Option<&'day Night>,
}

impl<'day> GoalPlan<'day> {
    const INN_DOOR: Distance = Distance::new(60.0);
    const HANDFUL_LENGTH: usize = 10;
    const GOAL_WHY_LENGTH: usize = 400;
    const INN_WHY_LENGTH: usize = 200;
    const NEIGHBOUR_PAIR_LENGTH: usize = 2;

    #[must_use]
    pub fn new(
        day: DayNumber,
        legs: &'day [Leg],
        aims: &'day [Aim],
        night: Option<&'day Night>,
    ) -> GoalPlan<'day> {
        GoalPlan {
            day,
            legs,
            aims,
            night,
        }
    }

    #[must_use]
    pub fn goals(&self) -> Vec<Goal> {
        let mut endings = self.endings();
        endings.dedup_by_key(|ending| ending.last());
        endings
            .into_iter()
            .filter(|ending| usize::from(ending.last()) < self.legs.len())
            .scan(LegIndex::default(), |first, ending| {
                let start = *first;
                *first = ending.last().incremented();
                Some(ending.starting_at(start))
            })
            .collect()
    }

    fn overlong(&self) -> impl Iterator<Item = Finding> {
        let aims = self.aims.iter().filter_map(|aim| {
            let length = AsRef::<Content>::as_ref(aim).char_length();
            (length > Self::GOAL_WHY_LENGTH).then(|| {
                Finding::on_leg(
                    self.day,
                    aim.until(),
                    Problem::TooLong {
                        label: Label::new("a goal's why"),
                        length,
                        length_limit: Self::GOAL_WHY_LENGTH,
                    },
                )
            })
        });
        let inn = self.night.and_then(|night| {
            let length = AsRef::<Content>::as_ref(night).char_length();
            (length > Self::INN_WHY_LENGTH).then(|| {
                Finding::on_day(
                    self.day,
                    Problem::TooLong {
                        label: Label::new("the night's why"),
                        length,
                        length_limit: Self::INN_WHY_LENGTH,
                    },
                )
            })
        });
        aims.chain(inn)
    }

    fn misplaced(&self, aim: &Aim) -> Option<Finding> {
        self.legs
            .get(usize::from(aim.until()))
            .map_or(Some(Problem::GoalPastEnd), |leg| {
                (leg.mode() != Mode::Walk).then_some(Problem::GoalOnRide)
            })
            .map(|problem| Finding::on_leg(self.day, aim.until(), problem))
    }

    fn endings(&self) -> Vec<Ending> {
        let rides = self
            .legs
            .iter()
            .zip(self.legs.iter().skip(1).map(Some).chain([None]))
            .enumerate()
            .filter(|(_, (leg, _))| leg.mode() != Mode::Walk)
            .filter(|(_, (_, next))| next.is_none_or(|next| next.mode() == Mode::Walk))
            .map(|(position, (leg, _))| {
                Ending::new(
                    LegIndex::new(position),
                    Purpose::Ride(leg.mode()),
                    leg.alighting()
                        .unwrap_or(leg.line_text(LegLine::Way).shown())
                        .clone(),
                    leg.line_text(LegLine::Why).clone(),
                )
            });
        let inns = self.night.into_iter().flat_map(|night| {
            self.legs
                .iter()
                .enumerate()
                .filter(|(_, leg)| leg.mode() == Mode::Walk)
                .filter(move |(_, leg)| {
                    leg.point(End::Finish).distance_to(Coordinate::from(night)) <= Self::INN_DOOR
                })
                .map(move |(position, _)| {
                    Ending::new(
                        LegIndex::new(position),
                        Purpose::Inn,
                        night[Caption::Name].clone(),
                        AsRef::<Content>::as_ref(night).clone(),
                    )
                })
        });
        let aims = self.aims.iter().map(|aim| {
            Ending::new(
                aim.until(),
                Purpose::Aim(aim.reason().clone()),
                aim[Caption::Name].clone(),
                AsRef::<Content>::as_ref(aim).clone(),
            )
        });
        let mut endings: Vec<Ending> = rides.chain(inns).chain(aims).collect();
        endings.sort_by_key(Ending::last);
        endings
    }
}

impl From<&GoalPlan<'_>> for Vec<Finding> {
    fn from(plan: &GoalPlan<'_>) -> Vec<Finding> {
        let endings = plan.endings();
        let shared = endings
            .windows(GoalPlan::NEIGHBOUR_PAIR_LENGTH)
            .filter(|pair| pair[0].last() == pair[1].last())
            .map(|pair| Finding::on_leg(plan.day, pair[0].last(), Problem::GoalsShareLeg));
        let misplaced = plan.aims.iter().filter_map(|aim| plan.misplaced(aim));
        let unordered = plan
            .aims
            .windows(GoalPlan::NEIGHBOUR_PAIR_LENGTH)
            .filter(|pair| pair[1].until() <= pair[0].until())
            .map(|pair| Finding::on_leg(plan.day, pair[1].until(), Problem::GoalsOutOfOrder));
        let last = LegIndex::new(plan.legs.len().saturating_sub(1));
        let reached = endings.last().map(Ending::last);
        let unfinished = (reached < Some(last)).then(|| {
            let first = reached.map_or(LegIndex::default(), newtype::Counter::incremented);
            Finding::on_leg(plan.day, first, Problem::NoGoal { last })
        });
        let length = plan.goals().len();
        let crowded = (length > GoalPlan::HANDFUL_LENGTH).then(|| {
            Finding::on_day(
                plan.day,
                Problem::TooManyGoals {
                    length,
                    length_limit: GoalPlan::HANDFUL_LENGTH,
                },
            )
        });
        let looped = plan
            .goals()
            .into_iter()
            .flat_map(|goal| LoopCheck::new(plan.day, plan.legs, &goal).findings());
        shared
            .chain(misplaced)
            .chain(unordered)
            .chain(unfinished)
            .chain(crowded)
            .chain(plan.overlong())
            .chain(looped)
            .collect()
    }
}

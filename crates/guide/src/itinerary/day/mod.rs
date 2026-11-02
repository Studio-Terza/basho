mod caption;
pub mod goal;
mod landing;
mod leg_lines;
mod night;

use crate::content::{Content, ContentList};
use crate::geo::Coordinate;
use crate::words::{Label, StarName};
use newtype::Counter;
use serde::Deserialize;
use std::ops::Index;

use crate::clock::Pause;
use crate::clock::{CalendarDate, ClockTime, Minutes, Sunset, TripZone};
use crate::end::End;
use crate::evidence::Evidence;
use crate::geo::Distance;
use crate::itinerary::DayNumber;
use crate::itinerary::Problem;
use crate::itinerary::arc_number::ArcNumber;
use crate::itinerary::day::goal::{Aim, Goal, GoalPlan, Purpose};
use crate::itinerary::finding::Finding;
use crate::leg::LegIndex;
use crate::leg::{Leg, Mode};
use crate::leg_line::LegLine;
use crate::place::RubyText;
use crate::reading::Reading;
use crate::readout::DayCatch;
use crate::times::{ListedTime, PlanTime};
use crate::way::{LegPath, RoadNetwork};
use landing::Landing;

pub use caption::Caption;
pub use leg_lines::LegLines;
pub use night::Night;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Day {
    number: DayNumber,
    date: CalendarDate,
    arc: ArcNumber,
    title: Content,
    japanese: Content,
    #[as_ref]
    why: Content,
    leave_from: Option<ListedTime>,
    landing: Option<Landing>,
    night: Option<Night>,
    legs: Vec<Leg>,
    #[serde(default, rename = "goals")]
    aims: Vec<Aim>,
    #[serde(skip)]
    goals: Vec<Goal>,
    #[serde(skip)]
    catch: Option<DayCatch>,
    #[serde(skip)]
    zone: TripZone,
    #[serde(skip)]
    spans_before: Vec<Minutes>,
    #[serde(skip)]
    broken_lines: Vec<Finding>,
}

impl Day {
    const JOIN_GAP: Distance = Distance::new(120.0);
    const SLACK_MINUTES: Minutes = Minutes::new(3);
    const WHY_LENGTH: usize = 300;
    const LEG_PAIR_LENGTH: usize = 2;

    #[must_use]
    pub fn assembled(self, zone: TripZone, lines: &LegLines, catch: Option<DayCatch>) -> Day {
        let network = RoadNetwork::from(lines);
        let paths: Vec<Result<LegPath, Problem>> = (0..self.legs.len())
            .map(|position| lines.path(LegIndex::new(position), &network))
            .collect();
        let broken_lines = paths
            .iter()
            .enumerate()
            .filter_map(|(position, path)| {
                path.as_ref()
                    .err()
                    .map(|problem| Finding::on_leg(self.number, LegIndex::new(position), problem))
            })
            .collect();
        let legs = self
            .legs
            .into_iter()
            .zip(paths)
            .map(|(leg, path)| leg.attached(&path.unwrap_or_default()))
            .collect::<Vec<Leg>>();
        let spans_before = std::iter::once(Minutes::ZERO)
            .chain(legs.iter().scan(Minutes::ZERO, |total, leg| {
                *total = *total
                    + if leg.mode() == Mode::Walk {
                        leg.minutes() + leg.paused(Pause::Along) + leg.paused(Pause::AtEnd)
                    } else {
                        Minutes::ZERO
                    };
                Some(*total)
            }))
            .collect();
        let goals = GoalPlan::new(self.number, &legs, &self.aims, self.night.as_ref()).goals();
        Day {
            legs,
            goals,
            catch,
            zone,
            spans_before,
            broken_lines,
            ..self
        }
    }

    #[must_use]
    pub fn zone(&self) -> TripZone {
        self.zone
    }

    #[must_use]
    pub fn catch(&self) -> Option<&DayCatch> {
        self.catch.as_ref()
    }

    #[must_use]
    pub fn follows(&self, previous: &Day) -> bool {
        self.number.number() == previous.number.number().wrapping_add(1)
            && self.date.is_day_after(previous.date)
    }

    #[must_use]
    pub fn leg(&self, index: LegIndex) -> Option<&Leg> {
        self.legs.get(usize::from(index))
    }

    #[must_use]
    pub fn branch_legs(&self, index: LegIndex) -> Vec<LegIndex> {
        let ride = self
            .leg(index)
            .filter(|leg| leg.mode() == Mode::Walk)
            .and(self.leg(index.incremented()))
            .filter(|next| next.mode() != Mode::Walk)
            .map(|_| index.incremented());
        [index]
            .into_iter()
            .chain(ride)
            .filter(|found| {
                self.leg(*found)
                    .is_some_and(|leg| !leg.branches().is_empty())
            })
            .collect()
    }

    #[must_use]
    pub fn number(&self) -> DayNumber {
        self.number
    }

    #[must_use]
    pub fn date(&self) -> CalendarDate {
        self.date
    }

    #[must_use]
    pub fn arc(&self) -> ArcNumber {
        self.arc
    }

    #[must_use]
    pub fn aims(&self) -> &[Aim] {
        &self.aims
    }

    #[must_use]
    pub fn title(&self) -> &Content {
        &self.title
    }

    #[must_use]
    pub fn leave_from(&self) -> Option<ClockTime> {
        self.leave_from.map(ClockTime::from).or_else(|| {
            self.landing
                .as_ref()
                .map(|landing| landing.time(End::Finish))
        })
    }

    #[must_use]
    pub fn landing(&self) -> Option<&Landing> {
        self.landing.as_ref()
    }

    #[must_use]
    pub fn night(&self) -> Option<&Night> {
        self.night.as_ref()
    }

    #[must_use]
    pub fn legs(&self) -> &[Leg] {
        &self.legs
    }

    #[must_use]
    pub fn sunset(&self) -> Option<Sunset> {
        self.night
            .as_ref()
            .map(Coordinate::from)
            .or_else(|| self.legs.last().map(|leg| leg.point(End::Finish)))
            .map(|place| Sunset::new(self.date, place, self.zone))
    }

    #[must_use]
    pub fn goals(&self) -> &[Goal] {
        &self.goals
    }

    #[must_use]
    pub fn goal_starts(&self) -> Vec<LegIndex> {
        self.goals.iter().map(|goal| goal.leg(End::Start)).collect()
    }

    #[must_use]
    pub fn goal_holding(&self, leg: LegIndex) -> Option<&Goal> {
        self.goals.iter().find(|goal| goal.holds(leg))
    }

    #[must_use]
    pub fn goal_evidence(&self, goal: &Goal) -> Option<&Evidence> {
        self.night
            .as_ref()
            .filter(|_| matches!(goal.purpose(), Purpose::Inn))
            .map(AsRef::as_ref)
    }

    #[must_use]
    pub fn claims(&self, point: &StarName) -> bool {
        self.goals
            .iter()
            .flat_map(|goal| goal.purpose().claims())
            .any(|name| name == point)
    }

    #[must_use]
    pub fn legs_toward(&self, goal: &Goal) -> &[Leg] {
        self.legs
            .get(usize::from(goal.leg(End::Start))..=usize::from(goal.leg(End::Finish)))
            .unwrap_or_default()
    }

    #[must_use]
    pub fn span_before(&self, index: LegIndex) -> Minutes {
        self.spans_before
            .get(usize::from(index).min(self.legs.len()))
            .copied()
            .unwrap_or_default()
    }

    #[must_use]
    pub fn walked(&self) -> Distance {
        self.legs
            .iter()
            .filter(|leg| leg.mode() == Mode::Walk)
            .map(Leg::length)
            .sum()
    }

    fn apart_legs(&self) -> Vec<Finding> {
        self.legs
            .windows(Self::LEG_PAIR_LENGTH)
            .enumerate()
            .filter(|(_, pair)| {
                pair[0]
                    .segments()
                    .last()
                    .zip(pair[1].segments().first())
                    .is_some_and(|(before, after)| before.run_into(after).is_err())
            })
            .map(|(index, _)| {
                Finding::on_leg(
                    self.number,
                    LegIndex::new(index),
                    Problem::LegsApart {
                        next: LegIndex::new(index + 1),
                    },
                )
            })
            .collect()
    }

    fn deadline_problems(&self) -> Vec<Problem> {
        let start = self.legs.first().map(|leg| leg.listed(End::Start));
        let unanchored = self
            .legs
            .first()
            .filter(|leg| leg.mode() != Mode::Walk && self.leave_from().is_none())
            .map(|_| Problem::RideFirstWithNoStart);
        let early = self
            .leave_from()
            .zip(start)
            .filter(|(leave_from, start)| *leave_from > ClockTime::from(*start))
            .map(|(leave_from, start)| Problem::LeaveAfterStart {
                leave_from: ListedTime::Planned(PlanTime::new(leave_from)),
                start,
            });
        let late = self
            .night
            .as_ref()
            .zip(self.legs.last())
            .filter(|(night, last)| last.time(End::Finish) > night.check_in_by())
            .map(|(night, last)| Problem::PastDeadline {
                place: night[Caption::Name].text(Reading::Written),
                arrival: last.listed(End::Finish),
                deadline: night.listed_check_in(),
            });
        let visits = self.legs.windows(Self::LEG_PAIR_LENGTH).filter_map(|pair| {
            let boards = pair[1].time(End::Start);
            let target = pair[1].boarding_margin()?.target(boards);
            let leaves = pair[0]
                .time(End::Finish)
                .later_by(pair[0].paused(Pause::AtEnd));
            (pair[0].mode() == Mode::Walk && leaves > target && leaves < boards).then(|| {
                Problem::PastDeadline {
                    place: pair[1].line_text(LegLine::Way).text(Reading::Written),
                    arrival: ListedTime::Planned(PlanTime::new(leaves)),
                    deadline: ListedTime::Planned(PlanTime::new(target)),
                }
            })
        });
        early
            .into_iter()
            .chain(unanchored)
            .chain(visits)
            .chain(late)
            .collect()
    }
}

impl From<&Day> for Vec<Finding> {
    fn from(day: &Day) -> Vec<Finding> {
        let legs = day
            .legs
            .iter()
            .enumerate()
            .flat_map(|(index, leg)| leg.findings(day.number, LegIndex::new(index)));
        let gaps = day
            .legs
            .windows(Day::LEG_PAIR_LENGTH)
            .enumerate()
            .filter_map(|(index, pair)| {
                let gap = pair[0]
                    .point(End::Finish)
                    .distance_to(pair[1].point(End::Start));
                (gap > Day::JOIN_GAP).then(|| {
                    Finding::on_leg(
                        day.number,
                        LegIndex::new(index),
                        Problem::GapToNext {
                            gap,
                            next: LegIndex::new(index + 1),
                        },
                    )
                })
            });
        let overlaps = day
            .legs
            .windows(Day::LEG_PAIR_LENGTH)
            .enumerate()
            .filter_map(|(index, pair)| {
                let spare = pair[0]
                    .time(End::Finish)
                    .minutes_until(pair[1].time(End::Start))
                    - pair[0].paused(Pause::AtEnd);
                (spare < -Day::SLACK_MINUTES).then(|| {
                    Finding::on_leg(
                        day.number,
                        LegIndex::new(index),
                        Problem::StartsBeforeDwell {
                            next: LegIndex::new(index + 1),
                            early: -spare,
                        },
                    )
                })
            });
        let transfers = day
            .legs
            .windows(Day::LEG_PAIR_LENGTH)
            .enumerate()
            .filter(|(_, pair)| pair[1].mode() != Mode::Walk)
            .filter(|(_, pair)| (pair[0].mode() != Mode::Walk) != pair[1].transfer().is_some())
            .map(|(index, _)| {
                Finding::on_leg(
                    day.number,
                    LegIndex::new(index + 1),
                    Problem::TransferMismatch,
                )
            });
        let goals = Vec::<Finding>::from(&GoalPlan::new(
            day.number,
            &day.legs,
            &day.aims,
            day.night.as_ref(),
        ));
        let why = (day.why.char_length() > Day::WHY_LENGTH).then(|| {
            Finding::on_day(
                day.number,
                Problem::TooLong {
                    label: Label::new("the day's why"),
                    length: day.why.char_length(),
                    length_limit: Day::WHY_LENGTH,
                },
            )
        });
        legs.chain(day.broken_lines.iter().cloned())
            .chain(gaps)
            .chain(day.apart_legs())
            .chain(overlaps)
            .chain(transfers)
            .chain(goals)
            .chain(why)
            .chain(
                day.legs
                    .is_empty()
                    .then(|| Finding::on_day(day.number, Problem::NoLegs)),
            )
            .chain(
                day.deadline_problems()
                    .into_iter()
                    .map(|problem| Finding::on_day(day.number, problem)),
            )
            .collect()
    }
}

impl Index<Caption> for Day {
    type Output = RubyText;

    fn index(&self, caption: Caption) -> &RubyText {
        match caption {
            Caption::Name => &self.japanese,
        }
    }
}

impl<'day> From<&'day Day> for ContentList<'day> {
    fn from(day: &'day Day) -> ContentList<'day> {
        let inn = day.night.iter().flat_map(|night| {
            std::iter::once(AsRef::<Content>::as_ref(night))
                .chain(AsRef::<Evidence>::as_ref(night).reason())
                .chain(night.notes().iter().flat_map(ContentList::from))
        });
        [&day.title, &day.japanese, &day.why]
            .into_iter()
            .chain(inn)
            .chain(day.aims.iter().map(AsRef::as_ref))
            .chain(day.aims.iter().flat_map(|aim| aim.reason().sight_names()))
            .chain(day.legs.iter().flat_map(ContentList::from))
            .chain(day.catch.iter().flat_map(ContentList::from))
            .collect()
    }
}

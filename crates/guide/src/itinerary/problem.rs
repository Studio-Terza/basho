use crate::words::{Label, Slug, Spelling, StarName};
use std::fmt;

use crate::clock::Minutes;
use crate::geo::{Distance, Speed};
use crate::itinerary::arc_number::ArcNumber;
use crate::leg::LegIndex;
use crate::times::{ListedTime, PlanTime};
use crate::way::NodeId;

#[derive(Clone, Debug, PartialEq)]
pub enum Problem {
    NoDays,
    NoLegs,
    DayOutOfOrder,
    StraySection {
        section: Label,
    },
    StrayRoute,
    DuplicatePlace {
        slug: Slug,
    },
    DuplicateArc {
        arc: ArcNumber,
    },
    UnknownPlace {
        slug: Slug,
    },
    TooLong {
        label: Label,
        length: usize,
        length_limit: usize,
    },
    NoBoarding,
    NoRoad,
    NoAlighting,
    NoRouteLine,
    WalkWithoutSegments,
    SegmentsOnRide,
    ShortMappedSegment,
    SegmentsApart,
    NodeOnNoWay {
        node: NodeId,
    },
    NoConnection {
        from: NodeId,
        to: NodeId,
    },
    EndOffWay {
        off: Distance,
    },
    LengthMismatch {
        stated: Distance,
        measured: Distance,
    },
    EndMiss {
        miss: Distance,
    },
    Pace {
        length: Distance,
        minutes: Minutes,
        speed: Speed,
    },
    Timing {
        start: ListedTime,
        end: ListedTime,
        planned: Minutes,
        minutes: Minutes,
        stopped: Minutes,
        accounted: Minutes,
    },
    StopsOutOfOrder {
        stop: Distance,
    },
    StopOutside {
        stop: Distance,
        length: Distance,
    },
    StopOffLine {
        name: Spelling,
        off: Distance,
    },
    GapToNext {
        gap: Distance,
        next: LegIndex,
    },
    StartsBeforeDwell {
        next: LegIndex,
        early: Minutes,
    },
    ArcMissesDay {
        arc: ArcNumber,
    },
    GoalsShareLeg,
    GoalPastEnd,
    GoalOnRide,
    GoalsOutOfOrder,
    LoopTooShort,
    LoopOutsideGoal,
    LoopOutOfOrder,
    LoopTimeOffLeg {
        planned: PlanTime,
        arrival: ListedTime,
    },
    NoGoal {
        last: LegIndex,
    },
    TooManyGoals {
        length: usize,
        length_limit: usize,
    },
    UnknownStarPoint {
        name: StarName,
    },
    DuplicateStarName {
        name: StarName,
    },
    UnclaimedStarPoint {
        name: StarName,
        off: Distance,
    },
    UncuedTime {
        trigger: Spelling,
    },
    DepartedOnWalk,
    PastDeadline {
        place: Spelling,
        arrival: ListedTime,
        deadline: ListedTime,
    },
    RideFirstWithNoStart,
    LeaveAfterStart {
        leave_from: ListedTime,
        start: ListedTime,
    },
    TourOnRide,
    RideTermsOnWalk,
    PayMissing,
    TransferMismatch,
    TourCheckInAfterStart {
        check_in: ListedTime,
    },
    BackupRideOutOfOrder {
        leaves: ListedTime,
        arrives: ListedTime,
    },
    NameInWords {
        name: Spelling,
    },
    UnknownEntry {
        name: Spelling,
    },
    UnknownSpot {
        name: Spelling,
    },
}

impl fmt::Display for Problem {
    #[expect(clippy::too_many_lines, reason = "one arm per problem")]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Problem::TooLong {
                label,
                length,
                length_limit,
            } => write!(
                formatter,
                "{label} is {length} characters; keep it to {length_limit}"
            ),
            Problem::NameInWords { name } => {
                write!(formatter, "the words name {name}, a known entry; write it as an entry part")
            }
            Problem::UnknownEntry { name } => {
                write!(formatter, "an entry part names {name}, and no entry has that name")
            }
            Problem::UnknownSpot { name } => {
                write!(formatter, "a place part names {name}, and no stop or feature has that name")
            }
            Problem::NoRoad => {
                formatter.write_str("a walking leg needs `road`, what the road is like")
            }
            Problem::NoBoarding => {
                formatter.write_str("a transit leg needs `from`, where you board")
            }
            Problem::NoAlighting => {
                formatter.write_str("a transit leg needs `to`, the stop where you get off")
            }
            Problem::NoRouteLine => formatter.write_str("has no line in the day's [[routes]]"),
            Problem::WalkWithoutSegments => {
                formatter.write_str("a walking line needs `segments`, mapped or no_mapped_path")
            }
            Problem::SegmentsOnRide => formatter.write_str("a ride's line has `coords`, not `segments`"),
            Problem::ShortMappedSegment => formatter.write_str("a mapped segment needs two nodes"),
            Problem::SegmentsApart => formatter.write_str(
                "two segments do not meet; put a no_mapped_path segment between them",
            ),
            Problem::NodeOnNoWay { node } => write!(formatter, "node {node} is on no way"),
            Problem::NoConnection { from, to } => {
                write!(formatter, "no way joins node {from} to node {to}")
            }
            Problem::EndOffWay { off } => {
                write!(formatter, "a segment end is {off} from its way")
            }
            Problem::NoDays => formatter.write_str("the trip has no days"),
            Problem::NoLegs => formatter.write_str("the day has no legs"),
            Problem::DayOutOfOrder => {
                formatter.write_str("day numbers and dates must rise one day after the other")
            }
            Problem::StraySection { section } => {
                write!(formatter, "[[{section}]] names no day, or names one twice")
            }
            Problem::StrayRoute => {
                formatter.write_str("a [[routes]] line is keyed to a leg the day does not have")
            }
            Problem::DuplicatePlace { slug } => {
                write!(formatter, "place {slug:?} is listed more than once")
            }
            Problem::DuplicateArc { arc } => {
                write!(formatter, "arc {arc} is listed more than once")
            }
            Problem::UnknownPlace { slug } => {
                write!(formatter, "place {slug:?} is not in [[places]]")
            }
            Problem::LengthMismatch { stated, measured } => write!(
                formatter,
                "km is {:.2} but its line measures {:.2}",
                stated.kilometers(),
                measured.kilometers()
            ),
            Problem::EndMiss { miss } => write!(
                formatter,
                "`at` is {:.0} m from the end of its line",
                miss.meters()
            ),
            Problem::Pace {
                length,
                minutes,
                speed,
            } => write!(
                formatter,
                "{:.2} km in {minutes} is {speed}",
                length.kilometers()
            ),
            Problem::Timing {
                start,
                end,
                planned,
                minutes,
                stopped,
                accounted,
            } => write!(
                formatter,
                "{start} → {end} is {planned}, but minutes ({minutes}) plus stops along the way ({stopped}) make {accounted}"
            ),
            Problem::StopsOutOfOrder { stop } => write!(
                formatter,
                "stops along the way are out of order at km {:.2}",
                stop.kilometers()
            ),
            Problem::StopOutside { stop, length } => write!(
                formatter,
                "a stop at km {:.2} is outside this {:.2} km leg",
                stop.kilometers(),
                length.kilometers()
            ),
            Problem::StopOffLine { name, off } => {
                write!(formatter, "{name} is {off} from the line")
            }
            Problem::GapToNext { gap, next } => write!(
                formatter,
                "ends {:.0} m from where leg {next} starts",
                gap.meters()
            ),
            Problem::StartsBeforeDwell { next, early } => write!(
                formatter,
                "leg {next} starts {early} before this leg ends and its dwell is over"
            ),
            Problem::ArcMissesDay { arc } => write!(formatter, "arc {arc} doesn't list this day"),
            Problem::GoalsShareLeg => {
                formatter.write_str("two goals end on this leg; give each its own")
            }
            Problem::GoalPastEnd => formatter.write_str("a goal ends after the day's last leg"),
            Problem::GoalOnRide => formatter
                .write_str("a goal ends on a ride; rides are goals already, so end it on a walk"),
            Problem::GoalsOutOfOrder => {
                formatter.write_str("goals are listed out of order; list them by `until`")
            }
            Problem::LoopTooShort => {
                formatter.write_str("a loop lists one sight; give it two or more, or use `sight`")
            }
            Problem::LoopOutsideGoal => {
                formatter.write_str("a loop sight names a leg outside the goal")
            }
            Problem::LoopOutOfOrder => {
                formatter.write_str("loop sights are listed out of order; list them by `leg`")
            }
            Problem::LoopTimeOffLeg { planned, arrival } => write!(
                formatter,
                "the loop plans {planned} here, but the leg arrives at {arrival}"
            ),
            Problem::NoGoal { last } => write!(
                formatter,
                "legs from here to {last} reach no goal; end them at a goal, a ride or the inn"
            ),
            Problem::TooManyGoals {
                length,
                length_limit,
            } => write!(
                formatter,
                "{length} goals is more than a handful; keep it to {length_limit}"
            ),
            Problem::UnknownStarPoint { name } => {
                write!(formatter, "{name:?} isn't a starred point on this day")
            }
            Problem::DuplicateStarName { name } => {
                write!(formatter, "two starred points share the short name {name:?}")
            }
            Problem::UnclaimedStarPoint { name, off } => write!(
                formatter,
                "the walk passes {name} ({off} off the line), so a star goal must claim it"
            ),
            Problem::UncuedTime { trigger } => write!(
                formatter,
                "the branch {trigger:?} names a clock time; give it `when = {{ after = .. }}`, `{{ until = .. }}` or \"departed\""
            ),
            Problem::DepartedOnWalk => {
                formatter.write_str("a branch cued \"departed\" is on a walk; only a ride departs")
            }
            Problem::PastDeadline {
                place,
                arrival,
                deadline,
            } => write!(
                formatter,
                "the plan reaches {place} at {arrival}, after its deadline {deadline}"
            ),
            Problem::RideFirstWithNoStart => formatter.write_str(
                "the day starts with a ride; give it `leave_from` or `landing`, so the plan knows when you reach it",
            ),
            Problem::LeaveAfterStart { leave_from, start } => write!(
                formatter,
                "`leave_from` {leave_from} is after the first leg starts at {start}"
            ),
            Problem::TourOnRide => formatter.write_str("a `tour` is on a ride; put it on a walk"),
            Problem::RideTermsOnWalk => formatter
                .write_str("`pay`, `margin` or `transfer` is on a walk; only a ride has them"),
            Problem::PayMissing => {
                formatter.write_str("a ride has no `pay`; say if it is `stored_value`, `held` or `buy`")
            }
            Problem::TransferMismatch => formatter.write_str(
                "a ride right after a ride needs `transfer`, and a ride after a walk has none",
            ),
            Problem::TourCheckInAfterStart { check_in } => write!(
                formatter,
                "the tour check-in {check_in} is after the tour starts"
            ),
            Problem::BackupRideOutOfOrder { leaves, arrives } => write!(
                formatter,
                "the backup ride leaves at {leaves} and arrives at {arrives}; it must leave after this ride and arrive after it leaves"
            ),
        }
    }
}

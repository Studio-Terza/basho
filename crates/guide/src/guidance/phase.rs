use crate::Stance;
use crate::clock::Minutes;
use crate::clock::TripTime;
use crate::end::End;
use crate::guidance::leg_progress::Reach;
use crate::guidance::time_check::TimeCheck;
use crate::guidance::tracking::Stage;
use crate::leg::{Leg, Mode};
use crate::readout::Status;
use crate::times::ShownTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Preview,
    Waiting { minutes: Minutes },
    Moving,
    Overdue,
    Missed,
    Done,
}

impl Phase {
    #[must_use]
    pub fn staged(self, mode: Option<Mode>) -> Stage {
        match self {
            Phase::Done => Stage::Finished,
            Phase::Moving => Stage::Under,
            Phase::Preview | Phase::Waiting { .. } | Phase::Overdue | Phase::Missed => {
                Self::boarding(mode)
            }
        }
    }

    fn boarding(mode: Option<Mode>) -> Stage {
        match mode {
            None | Some(Mode::Walk) => Stage::Under,
            Some(Mode::Train | Mode::Bus | Mode::Boat | Mode::Car) => Stage::Boarding,
        }
    }

    #[must_use]
    pub fn waiting_minutes(self) -> Option<Minutes> {
        match self {
            Phase::Waiting { minutes } => Some(minutes),
            Phase::Preview | Phase::Moving | Phase::Overdue | Phase::Missed | Phase::Done => None,
        }
    }

    #[must_use]
    pub fn at_platform(self) -> Option<Phase> {
        match self {
            Phase::Waiting { .. } | Phase::Overdue => Some(self),
            Phase::Preview | Phase::Moving | Phase::Missed | Phase::Done => None,
        }
    }

    #[must_use]
    pub fn stance(self) -> Stance {
        match self {
            Phase::Preview => Stance::Preview,
            Phase::Waiting { .. } => Stance::Waiting,
            Phase::Moving => Stance::Moving,
            Phase::Overdue | Phase::Missed => Stance::Missed,
            Phase::Done => Stance::Done,
        }
    }

    #[must_use]
    pub fn due(self, leg: &Leg, offset: Minutes) -> Option<ShownTime> {
        match self {
            Phase::Preview | Phase::Done => None,
            Phase::Waiting { .. } | Phase::Overdue | Phase::Missed => {
                Some(Self::when(leg, offset, End::Start))
            }
            Phase::Moving => Some(Self::when(leg, offset, End::Finish)),
        }
    }

    fn when(leg: &Leg, offset: Minutes, end: End) -> ShownTime {
        match leg.mode() {
            Mode::Walk => ShownTime::off_by(leg.listed(End::Finish), offset),
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => ShownTime::from(leg.listed(end)),
        }
    }

    #[must_use]
    pub fn ride_status(self, leg: &Leg) -> Option<Status> {
        match self {
            Phase::Waiting { .. } | Phase::Overdue => Self::waiting_for(leg),
            Phase::Moving => Self::riding(leg.mode()),
            Phase::Preview | Phase::Missed | Phase::Done => None,
        }
    }

    fn waiting_for(leg: &Leg) -> Option<Status> {
        match leg.mode() {
            Mode::Walk => None,
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Some(Status::Waiting {
                until: leg.listed(End::Start),
            }),
        }
    }

    fn riding(mode: Mode) -> Option<Status> {
        match mode {
            Mode::Walk => None,
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Some(Status::Riding(mode)),
        }
    }

    #[must_use]
    pub fn approach(self, mode: Mode) -> Approach {
        match self {
            Phase::Waiting { .. } | Phase::Overdue => Approach::from(mode),
            Phase::Preview | Phase::Moving | Phase::Missed | Phase::Done => Approach::Compare,
        }
    }

    #[must_use]
    pub fn unfinished<T>(self, found: Option<T>) -> Option<T> {
        match self {
            Phase::Done => None,
            Phase::Preview
            | Phase::Waiting { .. }
            | Phase::Moving
            | Phase::Overdue
            | Phase::Missed => found,
        }
    }

    #[must_use]
    pub fn apart<T>(self, found: Option<T>) -> Option<T> {
        match self {
            Phase::Waiting { .. } | Phase::Overdue => None,
            Phase::Preview | Phase::Moving | Phase::Missed | Phase::Done => found,
        }
    }
}

impl From<Reach> for Phase {
    fn from(reach: Reach) -> Phase {
        match reach {
            Reach::Arrived => Phase::Done,
            Reach::Underway => Phase::Moving,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Approach {
    Board,
    Compare,
}

impl From<Mode> for Approach {
    fn from(mode: Mode) -> Approach {
        match mode {
            Mode::Walk => Approach::Compare,
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Approach::Board,
        }
    }
}

impl Approach {
    #[must_use]
    pub fn time_check(self, due: TripTime, by: ShownTime, now: TripTime, leg: &Leg) -> TimeCheck {
        match self {
            Approach::Board => TimeCheck::boarding(due, by, now),
            Approach::Compare => TimeCheck::comparing(due, by, now, leg),
        }
    }
}

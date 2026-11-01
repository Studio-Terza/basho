use crate::clock::Minutes;
use crate::end::End;
use crate::guidance::tracking::Stage;
use crate::leg::{Leg, Mode};
use crate::readout::Status;
use crate::timeline::Stance;
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
        match (self, mode) {
            (Phase::Done, _) => Stage::Finished,
            (Phase::Moving, _) | (_, None | Some(Mode::Walk)) => Stage::Under,
            (
                Phase::Preview | Phase::Waiting { .. } | Phase::Overdue | Phase::Missed,
                Some(Mode::Train | Mode::Bus | Mode::Boat | Mode::Car),
            ) => Stage::Boarding,
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
        match (self, leg.mode()) {
            (Phase::Preview | Phase::Done, _) => None,
            (
                Phase::Waiting { .. } | Phase::Overdue | Phase::Missed,
                Mode::Train | Mode::Bus | Mode::Boat | Mode::Car,
            ) => Some(ShownTime::from(leg.listed(End::Start))),
            (Phase::Moving, Mode::Train | Mode::Bus | Mode::Boat | Mode::Car) => {
                Some(ShownTime::from(leg.listed(End::Finish)))
            }
            (
                Phase::Waiting { .. } | Phase::Overdue | Phase::Missed | Phase::Moving,
                Mode::Walk,
            ) => Some(ShownTime::off_by(leg.listed(End::Finish), offset)),
        }
    }

    #[must_use]
    pub fn ride_status(self, leg: &Leg) -> Option<Status> {
        match (self, leg.mode()) {
            (
                Phase::Waiting { .. } | Phase::Overdue,
                Mode::Train | Mode::Bus | Mode::Boat | Mode::Car,
            ) => Some(Status::Waiting {
                until: leg.listed(End::Start),
            }),
            (Phase::Moving, mode @ (Mode::Train | Mode::Bus | Mode::Boat | Mode::Car)) => {
                Some(Status::Riding(mode))
            }
            (Phase::Moving | Phase::Waiting { .. } | Phase::Overdue, Mode::Walk)
            | (Phase::Preview | Phase::Missed | Phase::Done, _) => None,
        }
    }
}

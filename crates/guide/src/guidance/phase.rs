use crate::Stance;
use crate::clock::Minutes;
use crate::end::End;
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
                Self::transit(mode).map_or(Stage::Under, |_| Stage::Boarding)
            }
        }
    }

    fn transit(mode: Option<Mode>) -> Option<Mode> {
        mode.filter(|mode| *mode != Mode::Walk)
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
        Some(leg.mode())
            .filter(|mode| *mode == Mode::Walk)
            .map_or_else(
                || ShownTime::from(leg.listed(end)),
                |_| ShownTime::off_by(leg.listed(End::Finish), offset),
            )
    }

    #[must_use]
    pub fn ride_status(self, leg: &Leg) -> Option<Status> {
        match self {
            Phase::Waiting { .. } | Phase::Overdue => {
                Self::transit(Some(leg.mode())).map(|_| Status::Waiting {
                    until: leg.listed(End::Start),
                })
            }
            Phase::Moving => Self::transit(Some(leg.mode())).map(Status::Riding),
            Phase::Preview | Phase::Missed | Phase::Done => None,
        }
    }
}

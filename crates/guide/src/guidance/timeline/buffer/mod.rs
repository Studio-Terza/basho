mod live_gap;
mod reaching;
use crate::buffer_state::BufferState;
use crate::clock::{ClockTime, Minutes};
use crate::guidance::timeline::backup::Backup;
use crate::guidance::timeline::deadline::Deadline;
use crate::guidance::timeline::footing::Footing;
use crate::guidance::timeline::here::Here;
use crate::times::Basis;
pub use live_gap::LiveGap;
pub use reaching::Reaching;

#[derive(Clone, Copy, Debug)]
pub struct Buffer<'day> {
    deadline: Deadline<'day>,
    projected: Minutes,
    floor: Option<Minutes>,
    seen: Option<Minutes>,
    footing: Footing,
    backup: Option<Backup>,
}

impl<'day> Buffer<'day> {
    #[must_use]
    pub fn new(deadline: Deadline<'day>, projected: Minutes) -> Buffer<'day> {
        Buffer {
            deadline,
            projected,
            floor: None,
            seen: None,
            footing: Footing::OnFoot,
            backup: None,
        }
    }

    #[must_use]
    pub fn floored(self, floor: Option<Minutes>) -> Buffer<'day> {
        Buffer { floor, ..self }
    }

    #[must_use]
    pub fn seen_from(self, here: Here) -> Buffer<'day> {
        Buffer {
            seen: here.clock(),
            ..self
        }
    }

    #[must_use]
    pub fn reaches_by(&self, time: ClockTime) -> Reaching {
        Reaching::from(self.projected.cmp(&time.since_midnight()))
    }

    #[must_use]
    pub fn seated(self, footing: Footing) -> Buffer<'day> {
        Buffer { footing, ..self }
    }

    #[must_use]
    pub fn backed_by(self, backup: Option<Backup>) -> Buffer<'day> {
        Buffer { backup, ..self }
    }

    #[must_use]
    pub fn deadline(&self) -> Deadline<'day> {
        self.deadline
    }

    #[must_use]
    pub fn projected(&self) -> ClockTime {
        ClockTime::at_hour(0).later_by(self.projected)
    }

    #[must_use]
    pub fn spare(&self) -> Minutes {
        self.boarding()
            .map_or_else(|| self.deadline.window().start, |_| self.deadline.event())
            .since_midnight()
            - self.projected
    }

    #[must_use]
    pub fn gap(&self) -> LiveGap {
        LiveGap::new(
            self.spare(),
            self.seen.map_or(Basis::PlanTimes, |seen| {
                Basis::PlanLeft((self.projected - seen).max(Minutes::ZERO))
            }),
        )
    }

    #[must_use]
    pub fn state(&self) -> BufferState {
        let spare = BufferState::for_spare(self.spare());
        let calm = Self::calmed(spare, self.footing);
        self.boarding().map_or(calm, |_| BufferState::Waiting)
    }

    fn calmed(state: BufferState, footing: Footing) -> BufferState {
        match state {
            BufferState::Tight => Self::eased(footing),
            BufferState::Comfortable | BufferState::Short | BufferState::Waiting => state,
        }
    }

    fn eased(footing: Footing) -> BufferState {
        match footing {
            Footing::Seated => BufferState::Comfortable,
            Footing::OnFoot => BufferState::Tight,
        }
    }

    #[must_use]
    pub fn boarding(&self) -> Option<Minutes> {
        self.floor
            .filter(|now| self.projected <= *now)
            .and_then(|now| self.reaches_by(self.deadline.event()).keep(now))
    }

    #[must_use]
    pub fn reaches_before_event(&self) -> Reaching {
        match self.floor {
            Some(_) => Reaching::Late,
            None => self.reaches_by(self.deadline.event()),
        }
    }

    #[must_use]
    pub fn backup(&self) -> Option<Backup> {
        self.backup
    }
}

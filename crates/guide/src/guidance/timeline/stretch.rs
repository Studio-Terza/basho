use std::ops::Range;

use crate::clock::{ClockTime, Minutes};
use crate::guidance::timeline::buffer::Buffer;
use crate::guidance::timeline::deadline::Deadline;
use crate::guidance::timeline::here::Here;
use crate::leg::LegIndex;

#[derive(Clone, Debug)]
pub struct Stretch<'day> {
    opener: LegIndex,
    closer: Option<LegIndex>,
    opens: Option<ClockTime>,
    start: Minutes,
    finish: Minutes,
    end: Option<Deadline<'day>>,
    cutoffs: Vec<Deadline<'day>>,
}

impl<'day> Stretch<'day> {
    #[must_use]
    pub fn opening(opener: LegIndex, opens: Option<ClockTime>, start: Minutes) -> Stretch<'day> {
        Stretch {
            opener,
            closer: None,
            opens,
            start,
            finish: start,
            end: None,
            cutoffs: Vec::new(),
        }
    }

    #[must_use]
    pub fn closed(
        self,
        closer: Option<LegIndex>,
        end: Option<Deadline<'day>>,
        finish: Minutes,
    ) -> Stretch<'day> {
        Stretch {
            closer,
            finish,
            end,
            ..self
        }
    }

    #[must_use]
    pub fn holding(self, cutoffs: Vec<Deadline<'day>>) -> Stretch<'day> {
        Stretch { cutoffs, ..self }
    }

    #[must_use]
    pub fn ending(self, end: Deadline<'day>) -> Stretch<'day> {
        Stretch {
            end: Some(end),
            ..self
        }
    }

    #[must_use]
    pub fn end(&self) -> Option<Deadline<'day>> {
        self.end
    }

    #[must_use]
    pub fn opener(&self) -> LegIndex {
        self.opener
    }

    #[must_use]
    pub fn closer(&self) -> Option<LegIndex> {
        self.closer
    }

    #[must_use]
    pub fn opens(&self) -> Option<ClockTime> {
        self.opens
    }

    #[must_use]
    pub fn start(&self) -> Minutes {
        self.start
    }

    #[must_use]
    pub fn holds(&self, leg: LegIndex) -> bool {
        self.opener <= leg && self.closer.is_none_or(|closer| leg < closer)
    }

    #[must_use]
    pub fn binding(
        &self,
        here: Here,
        floor: Option<Minutes>,
        first: LegIndex,
        last: LegIndex,
    ) -> Option<Buffer<'day>> {
        let end = self.end.map(|end| {
            let projected = here.projected(end.reach());
            Buffer::new(end, floor.map_or(projected, |floor| projected.max(floor)))
                .floored(floor)
                .seen_from(here)
        });
        self.cutoffs
            .iter()
            .filter(|deadline| here.ahead_of(deadline))
            .map(|deadline| {
                Buffer::new(*deadline, here.projected(deadline.reach())).seen_from(here)
            })
            .chain(end)
            .filter(|buffer| buffer.deadline().leg() >= first && buffer.deadline().leg() <= last)
            .min_by_key(Buffer::spare)
    }

    #[must_use]
    pub fn span(&self) -> Range<Minutes> {
        self.start..self.finish
    }
}

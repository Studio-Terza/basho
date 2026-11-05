use std::ops::Range;

use crate::clock::{ClockTime, Minutes};
use crate::deadline_kind::DeadlineKind;
use crate::evidence::Evidence;
use crate::haste::Haste;
use crate::leg::{Cutoff, LegIndex, Mode};
use crate::ruby_text::RubyText;
use crate::times::{ListedTime, PlanTime};

#[derive(Clone, Copy, Debug)]
pub struct Deadline<'day> {
    kind: DeadlineKind,
    mode: Mode,
    place: &'day RubyText,
    vouched: Option<&'day Evidence>,
    event: ListedTime,
    target: ListedTime,
    leg: LegIndex,
    reach: Minutes,
    haste: Haste,
}

impl<'day> Deadline<'day> {
    #[must_use]
    pub fn cutoff(
        cutoff: &'day Cutoff,
        place: &'day RubyText,
        leg: LegIndex,
        reach: Minutes,
    ) -> Deadline<'day> {
        Deadline {
            kind: cutoff.kind().deadline_kind(),
            mode: Mode::Walk,
            place: cutoff.name().unwrap_or(place),
            vouched: None,
            event: cutoff.listed_at(),
            target: cutoff.listed_at(),
            leg,
            reach,
            haste: Haste::DoNotStop,
        }
    }

    #[must_use]
    pub fn fixed(kind: DeadlineKind, place: &'day RubyText, leg: LegIndex) -> Deadline<'day> {
        Deadline {
            kind,
            mode: Mode::Walk,
            place,
            vouched: None,
            event: ListedTime::Planned(PlanTime::new(ClockTime::MIDNIGHT)),
            target: ListedTime::Planned(PlanTime::new(ClockTime::MIDNIGHT)),
            leg,
            reach: Minutes::ZERO,
            haste: Haste::DoNotStop,
        }
    }

    #[must_use]
    pub fn timed(self, event: ListedTime, target: ListedTime) -> Deadline<'day> {
        Deadline {
            event,
            target,
            ..self
        }
    }

    #[must_use]
    pub fn reached_at(self, reach: Minutes, mode: Mode) -> Deadline<'day> {
        Deadline {
            mode,
            reach,
            ..self
        }
    }

    #[must_use]
    pub fn vouched_by(self, evidence: &'day Evidence) -> Deadline<'day> {
        Deadline {
            vouched: Some(evidence),
            ..self
        }
    }

    #[must_use]
    pub fn urged(self, haste: Haste) -> Deadline<'day> {
        Deadline { haste, ..self }
    }

    #[must_use]
    pub fn kind(&self) -> DeadlineKind {
        self.kind
    }

    #[must_use]
    pub fn mode(&self) -> Mode {
        self.mode
    }

    #[must_use]
    pub fn place(&self) -> &'day RubyText {
        self.place
    }

    #[must_use]
    pub fn place_evidence(&self) -> Option<&'day Evidence> {
        self.vouched
    }

    #[must_use]
    pub fn event(&self) -> ClockTime {
        ClockTime::from(self.listed().end)
    }

    #[must_use]
    pub fn listed(&self) -> Range<ListedTime> {
        self.target..self.event
    }

    #[must_use]
    pub fn window(&self) -> Range<ClockTime> {
        ClockTime::from(self.target)..ClockTime::from(self.event)
    }

    #[must_use]
    pub fn leg(&self) -> LegIndex {
        self.leg
    }

    #[must_use]
    pub fn reach(&self) -> Minutes {
        self.reach
    }

    #[must_use]
    pub fn haste(&self) -> Haste {
        self.haste
    }
}

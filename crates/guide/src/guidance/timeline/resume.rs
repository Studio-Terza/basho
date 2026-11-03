use crate::clock::Minutes;
use crate::guidance::timeline::stretch::Stretch;
use crate::guidance::timeline::stretch_index::StretchIndex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resume {
    stretch: StretchIndex,
    time: Minutes,
}

impl Resume {
    #[must_use]
    pub fn new(stretch: StretchIndex, time: Minutes) -> Resume {
        Resume { stretch, time }
    }

    #[must_use]
    pub fn stretch(self) -> StretchIndex {
        self.stretch
    }

    #[must_use]
    pub fn time(self) -> Minutes {
        self.time
    }

    #[must_use]
    pub fn arrival_in(self, stretch: &Stretch<'_>) -> Minutes {
        let span = stretch.span();
        self.time + span.end - span.start
    }
}

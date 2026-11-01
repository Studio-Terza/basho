use guide::classed::Classed;
use std::fmt;

use super::shape::Shape;
use super::tone::Tone;
use layout::measure::MarkSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Look {
    shape: Shape,
    tone: Tone,
    size: MarkSize,
}

impl Look {
    #[must_use]
    pub fn new(shape: Shape, tone: Tone, size: MarkSize) -> Look {
        Look { shape, tone, size }
    }

    #[must_use]
    pub fn shape(self) -> Shape {
        self.shape
    }

    #[must_use]
    pub fn tone(self) -> Tone {
        self.tone
    }

    #[must_use]
    pub fn size(self) -> MarkSize {
        self.size
    }
}

impl fmt::Display for Look {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.tone.class(), self.size.class())
    }
}

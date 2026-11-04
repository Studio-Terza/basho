use guide::classed::Classed;
use std::fmt;

#[derive(newtype::Newtype, newtype::Display, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Glyph(char);

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toward {
    #[strum(serialize = "step-back")]
    Previous,
    #[strum(serialize = "step-next")]
    Next,
}

impl Toward {
    #[must_use]
    pub fn reversed(self) -> Toward {
        match self {
            Toward::Previous => Toward::Next,
            Toward::Next => Toward::Previous,
        }
    }

    #[must_use]
    pub fn glyph(self) -> Glyph {
        match self {
            Toward::Previous => Glyph::new('‹'),
            Toward::Next => Glyph::new('›'),
        }
    }

    pub(super) fn chosen<Chosen>(
        self,
        previous: impl FnOnce() -> Chosen,
        next: impl FnOnce() -> Chosen,
    ) -> Chosen {
        match self {
            Toward::Previous => previous(),
            Toward::Next => next(),
        }
    }
}

impl Classed for Toward {}

impl fmt::Display for Toward {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Toward::Previous => formatter.write_str("Previous step"),
            Toward::Next => formatter.write_str("Next step"),
        }
    }
}

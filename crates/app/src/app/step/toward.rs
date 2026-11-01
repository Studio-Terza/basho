use guide::classed::Classed;
use std::fmt;

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
    pub fn glyph(self) -> char {
        match self {
            Toward::Previous => '‹',
            Toward::Next => '›',
        }
    }
}

impl Classed for Toward {}

impl fmt::Display for Toward {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Toward::Previous => "Previous step",
            Toward::Next => "Next step",
        })
    }
}

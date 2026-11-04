use guide::words::Label;
use std::fmt;

use guide::Scale;

use guide::geo::Share;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenShare {
    MapLeast,
}

impl ScreenShare {
    pub const ALL: &'static [ScreenShare] = &[ScreenShare::MapLeast];
    const WHOLE: Scale = Scale::new(100.0);
    const MAP_LEAST: Share = Share::new(0.35);

    pub fn name(self) -> Label {
        match self {
            ScreenShare::MapLeast => Label::new("share-map-least"),
        }
    }

    fn share(self) -> Share {
        match self {
            ScreenShare::MapLeast => Self::MAP_LEAST,
        }
    }
}

impl fmt::Display for ScreenShare {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:.1}dvh",
            self.share().fraction() * Self::WHOLE.factor()
        )
    }
}

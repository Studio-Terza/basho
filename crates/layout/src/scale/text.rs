use guide::geo::Share;
use guide::map_frame::Pixels;
use guide::words::Label;

use crate::scale::token::Token;
use crate::scale::viewport::Viewport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Text {
    Body,
    Compact,
    Major,
    Town,
    Label,
    Minor,
    Least,
}

impl Text {
    pub const ASCENT: Share = Share::new(0.8);
    pub const DESCENT: Share = Share::new(0.25);
    pub const MIDDLE: Share = Share::new(0.35);
    pub const ADVANCE: Share = Share::new(0.62);
    pub const HEAVY_ADVANCE: Share = Share::new(0.7);
    const BODY: Pixels = Pixels::new(19.0);
    const COMPACT: Pixels = Pixels::new(15.0);
    const MAJOR: Pixels = Pixels::new(12.0);
    const TOWN: Pixels = Pixels::new(11.5);
    const LABEL: Pixels = Pixels::new(11.0);
    const MINOR: Pixels = Pixels::new(10.5);
    const LEAST: Pixels = Pixels::new(10.0);

    #[must_use]
    pub fn advance(self, glyph: Share) -> Pixels {
        self.design() * glyph.fraction()
    }

    fn least(self) -> Pixels {
        match self {
            Text::Body => Self::BODY,
            Text::Compact => Self::COMPACT,
            Text::Major => Self::MAJOR,
            Text::Town => Self::TOWN,
            Text::Label => Self::LABEL,
            Text::Minor => Self::MINOR,
            Text::Least => Self::LEAST,
        }
    }
}

impl Token for Text {
    const ALL: &'static [Text] = &[
        Text::Body,
        Text::Compact,
        Text::Major,
        Text::Town,
        Text::Label,
        Text::Minor,
        Text::Least,
    ];

    fn name(self) -> Label {
        Label::new(match self {
            Text::Body => "text-body",
            Text::Compact => "text-compact",
            Text::Major => "text-major",
            Text::Town => "text-town",
            Text::Label => "text-label",
            Text::Minor => "text-minor",
            Text::Least => "text-least",
        })
    }

    fn length(self, viewport: Viewport) -> Pixels {
        viewport.scaled(self.least()).max(self.least())
    }
}

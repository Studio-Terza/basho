mod focus;
mod jumps;
mod landing;
mod scroll;
use std::fmt;

pub use focus::Focus;
use jumps::Jumps;
pub use landing::Landing;
pub use scroll::Scroll;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Visit {
    focus: Focus,
    landing: Landing,
    jumps: Jumps,
}

impl Visit {
    #[must_use]
    pub fn focus(self) -> Focus {
        self.focus
    }

    #[must_use]
    pub fn landing(self) -> Landing {
        self.landing
    }

    #[must_use]
    pub fn jumped(self, focus: Focus, landing: Landing) -> Visit {
        Visit {
            focus,
            landing,
            jumps: self.jumps.next(),
        }
    }

    #[must_use]
    pub fn refocused(self, focus: Focus) -> Visit {
        Visit { focus, ..self }
    }

    #[must_use]
    pub fn restored(focus: Focus) -> Visit {
        let landing = focus
            .spot()
            .map_or(Landing::Now, |spot| Landing::Step(spot.step()));
        Visit::default().jumped(focus, landing)
    }

    #[must_use]
    pub fn home(self) -> Visit {
        self.jumped(Focus::Now, Landing::Now)
    }

    #[must_use]
    pub fn scrolled(self, scroll: Scroll) -> Visit {
        Visit {
            landing: self.landing.scrolled(scroll),
            ..self
        }
    }
}

impl fmt::Display for Visit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.jumps)
    }
}

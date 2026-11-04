use guide::plane::Pixels;
use guide::words::Label;

use crate::scale::Token;
use crate::scale::Viewport;
use crate::touch::Touch;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Key,
    Row,
    TabBar,
}

impl Token for Target {
    const ALL: &'static [Target] = &[Target::Key, Target::Row, Target::TabBar];

    fn name(self) -> Label {
        match self {
            Target::Key => Label::new("target-key"),
            Target::Row => Label::new("target-row"),
            Target::TabBar => Label::new("target-tab-bar"),
        }
    }

    fn length(self, viewport: Viewport) -> Pixels {
        viewport.scaled(self.least()).max(self.least())
    }
}

impl Target {
    const TAB_BAR: Pixels = Pixels::new(58.0);

    fn least(self) -> Pixels {
        match self {
            Target::Key | Target::Row => Touch::Tap.extent(),
            Target::TabBar => Self::TAB_BAR,
        }
    }
}

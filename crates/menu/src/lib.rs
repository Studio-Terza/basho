mod closed;
mod closing;
mod menu_step;
mod open;
mod pose;
mod pull;

use guide::geo::Share;
use guide::plane::Pixels;

use closed::Closed;
use closing::Closing;
use layout::Panel;
use layout::scale::Viewport;
pub use menu_step::MenuStep;
use open::Open;
pub use pose::PoseStyle;
use pose::{Follow, Pose, Reach};
pub use pull::{DragFrom, Pull};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Menu {
    Closed(Closed),
    Open(Open),
    Closing(Closing),
}

impl Menu {
    pub const CLOSED: Menu = Menu::Closed(Closed);
    pub const OPEN: Menu = Menu::Open(Open);

    #[must_use]
    pub fn width(viewport: Viewport) -> Pixels {
        Panel::Menu.width(viewport)
    }

    #[must_use]
    pub fn style(self, pull: Option<Pull>, viewport: Viewport) -> PoseStyle {
        let width = Self::width(viewport);
        pull.map_or_else(|| self.resting(width), |pull| self.pulled(pull, width))
            .styled(Menu::CLOSED.resting(width), width)
    }

    #[must_use]
    pub fn after(self, step: MenuStep) -> Menu {
        match self {
            Menu::Closed(closed) => step.on_closed(closed),
            Menu::Open(open) => step.on_open(open),
            Menu::Closing(closing) => step.on_closing(closing),
        }
    }

    fn resting(self, width: Pixels) -> Pose {
        match self {
            Menu::Open(_) => Pose::new(Share::new(1.0), Pixels::ZERO, Follow::Ease, Reach::Live),
            Menu::Closed(_) | Menu::Closing(_) => {
                Pose::new(Share::new(0.0), -width, Follow::Ease, Reach::Inert)
            }
        }
    }

    fn pulled(self, pull: Pull, width: Pixels) -> Pose {
        match self {
            Menu::Open(_) | Menu::Closed(_) => Pose::held(pull.reach(), width),
            Menu::Closing(_) => self.resting(width),
        }
    }
}

impl Default for Menu {
    fn default() -> Menu {
        Menu::CLOSED
    }
}

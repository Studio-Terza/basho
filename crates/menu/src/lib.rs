mod closed;
mod closing;
mod menu_step;
mod open;
mod pose;
mod pull;
mod state;

use guide::plane::Pixels;

use closed::Closed;
use closing::Closing;
use layout::Panel;
use layout::scale::Viewport;
pub use menu_step::MenuStep;
use open::Open;
use pose::Pose;
pub use pose::PoseStyle;
pub use pull::{DragFrom, Pull};
use state::State;

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
        let pose = match (self, pull) {
            (Menu::Open(_) | Menu::Closed(_), Some(pull)) => Pose::held(pull.reach(), width),
            (Menu::Open(open), None) => open.pose(width),
            (Menu::Closing(closing), _) => closing.pose(width),
            (Menu::Closed(closed), None) => closed.pose(width),
        };
        pose.styled(Closed.pose(width), width)
    }

    #[must_use]
    pub fn after(self, step: MenuStep) -> Menu {
        match (step, self) {
            (MenuStep::Open, Menu::Closed(closed)) => Menu::Open(closed.open()),
            (MenuStep::Open, Menu::Closing(closing)) => Menu::Open(closing.reopen()),
            (MenuStep::Close, Menu::Open(open)) => Menu::Closing(open.close()),
            (MenuStep::Settle, Menu::Closing(closing)) => Menu::Closed(closing.settle()),
            (MenuStep::Open, Menu::Open(_))
            | (MenuStep::Close, Menu::Closed(_) | Menu::Closing(_))
            | (MenuStep::Settle, Menu::Closed(_) | Menu::Open(_)) => self,
        }
    }
}

impl Default for Menu {
    fn default() -> Menu {
        Menu::CLOSED
    }
}

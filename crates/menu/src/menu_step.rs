use crate::Menu;
use crate::closed::Closed;
use crate::closing::Closing;
use crate::open::Open;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuStep {
    Open,
    Close,
    Settle,
}

impl MenuStep {
    pub(crate) fn on_closed(self, closed: Closed) -> Menu {
        match self {
            MenuStep::Open => Menu::Open(closed.open()),
            MenuStep::Close | MenuStep::Settle => Menu::Closed(closed),
        }
    }

    pub(crate) fn on_open(self, open: Open) -> Menu {
        match self {
            MenuStep::Close => Menu::Closing(open.close()),
            MenuStep::Open | MenuStep::Settle => Menu::Open(open),
        }
    }

    pub(crate) fn on_closing(self, closing: Closing) -> Menu {
        match self {
            MenuStep::Open => Menu::Open(closing.reopen()),
            MenuStep::Settle => Menu::Closed(closing.settle()),
            MenuStep::Close => Menu::Closing(closing),
        }
    }
}

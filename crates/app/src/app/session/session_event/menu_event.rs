use dioxus::prelude::*;
use layout::Motion;
use menu::{Menu, MenuStep};
use std::time::Duration;

use crate::app::session::Session;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuEvent {
    Open,
    Close,
}

impl MenuEvent {
    pub fn moved_menu(self, session: &Session) {
        match self {
            MenuEvent::Open => Self::opened(session.menu),
            MenuEvent::Close => Self::closed(session),
        }
    }

    fn opened(mut menu: Signal<Menu>) {
        let current = *menu.peek();
        menu.set(current.after(MenuStep::Open));
    }

    fn closed(session: &Session) {
        let mut menu = session.menu;
        let current = *menu.peek();
        let closed = current.after(MenuStep::Close);
        (closed != current).then(|| {
            menu.set(closed);
            spawn(async move {
                tokio::time::sleep(Duration::from(Motion::Slide)).await;
                let sliding = *menu.peek();
                menu.set(sliding.after(MenuStep::Settle));
            });
        });
    }
}

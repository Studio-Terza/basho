use dioxus::prelude::*;
use layout::Motion;
use menu::MenuStep;
use std::time::Duration;

use crate::app::session::Session;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuEvent {
    Open,
    Close,
}

impl MenuEvent {
    pub fn moved_menu(self, session: &Session) {
        let mut menu = session.menu;
        let current = *menu.peek();
        match self {
            MenuEvent::Open => menu.set(current.after(MenuStep::Open)),
            MenuEvent::Close => {
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
    }
}

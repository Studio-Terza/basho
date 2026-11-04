mod reach;

use dioxus::prelude::*;
use guide::classed::Classed;
use newtype::Newtype;
use reach::Reach;

use crate::app::session::Session;
use crate::app::step::Toward;

#[derive(Newtype)]
pub struct Stepper<'session> {
    session: &'session Session,
}

impl Stepper<'_> {
    pub fn render(self, toward: Toward) -> Element {
        let target = self.session.stepped_focus(toward);
        let session = self.session.clone();
        let reach = Reach::of_target(target.as_ref());
        rsx! {
            button { class: "stepper {toward.class()}", "aria-label": "{toward}",
                disabled: reach,
                onclick: move |_| target.into_iter().for_each(|spot| session.browse(spot)),
                "{toward.glyph()}"
            }
        }
    }
}

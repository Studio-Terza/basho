use dioxus::prelude::*;

use basho::app::screen::ScreenFrame;
use basho::app::session::Session;
use basho::app::session::staged::Staged;

use crate::ui_check::Stage;

pub struct Harness;

impl Harness {
    pub fn page(stage: &Stage) -> String {
        let mut dom = VirtualDom::new_with_props(Self::render, stage.clone());
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "dioxus hands root props by value"
    )]
    fn render(stage: Stage) -> Element {
        let itinerary = use_hook(|| stage.drawing().itinerary());
        let places = use_hook(|| stage.drawing().places());
        let overview = use_hook(|| stage.drawing().overview());
        let session = Session::staged(itinerary, places, overview, &stage);
        rsx! { {ScreenFrame::new(*stage.screen(), &session)} }
    }
}

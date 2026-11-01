pub mod history;
mod map_feed;
pub mod screen;
pub mod session;
pub mod step;

use dioxus::prelude::*;
use guide::Trip;

use map_feed::MapFeed;
use screen::{Screen, ScreenFrame};
use session::{Session, TripShelf};

pub struct App;

impl App {
    pub fn trip_page(trip: &Trip, shelf: TripShelf) -> Element {
        let session = Session::open(trip, shelf);
        MapFeed::start(&session, ScreenFrame::map_update);
        rsx! { {ScreenFrame::new(session.screen(), &session)} }
    }

    pub fn render() -> Element {
        let shelf = Session::launch_shelf(Self::trip_page);
        rsx! {
            document::Stylesheet { href: Screen::STYLE }
            {shelf.view()}
        }
    }
}

use guide::itinerary::DayNumber;
use guide::words::Slug;
use wiki::EntryKey;

use basho::app::screen::Screen;

use crate::ui_check::{Drawing, Harness, Stage};

const DAY: DayNumber = DayNumber::new(1);
const PLACE: &str = "eastfield";
const BLOCK: &str = "class=\"disagreement\"";
const LINK: &str = "class=\"link\"";

#[test]
fn where_the_accounts_differ_the_names_of_other_pages_are_links() {
    let drawing = Drawing::BuiltIn;
    let trip = drawing.itinerary();
    let day = trip.day_number(DAY).expect("a day");
    let page = drawing
        .wiki()
        .find(&EntryKey::Place(Slug::new(PLACE.to_owned())))
        .expect("the place has a page");
    let html = Harness::page(&Stage::on_screen(day, Screen::Wiki(page)));
    let blocks: Vec<&str> = html.split(BLOCK).skip(1).collect();
    assert_ne!(blocks.len(), 0);
    assert!(blocks.iter().any(|block| {
        block
            .split("</div>")
            .next()
            .unwrap_or_default()
            .contains(LINK)
    }));
}

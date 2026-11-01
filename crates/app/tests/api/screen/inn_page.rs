use guide::itinerary::DayNumber;
use wiki::EntryKey;

use basho::app::screen::Screen;

use crate::ui_check::{Drawing, Harness, Stage};

const DAY: DayNumber = DayNumber::new(1);
const MARK: &str = "class=\"unverified\"";

#[test]
fn an_inn_that_is_not_checked_carries_the_mark_on_its_own_page() {
    let drawing = Drawing::Second;
    let trip = drawing.itinerary();
    let day = trip.day_number(DAY).expect("a day");
    let inn = drawing
        .wiki()
        .find(&EntryKey::Inn(DAY))
        .expect("the inn has a page");
    let page = Harness::page(&Stage::on_screen(day, Screen::Wiki(inn)).drawn_with(drawing));
    assert!(page.contains("Mill Inn"));
    assert!(page.contains(MARK), "{page}");
}

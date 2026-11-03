use guide::day_number::DayNumber;

use basho::app::session::visit::Focus;
use basho::app::session::visit::Landing;
use basho::app::session::visit::Scroll;
use basho::app::session::visit::Visit;
use basho::app::step::Spot;
use basho::app::step::StepNumber;

const STEP: StepNumber = StepNumber::from_position(2);
const BROWSED: Focus = Focus::Browsing(Spot::new(DayNumber::new(4), STEP));

#[test]
fn a_rebuilt_page_keeps_the_focus_and_lands_on_its_step() {
    let restored = [BROWSED, Focus::Now].map(Visit::restored);
    assert_eq!(
        restored.map(|visit| (visit.focus(), visit.landing())),
        [(BROWSED, Landing::Step(STEP)), (Focus::Now, Landing::Now)]
    );
}

#[test]
fn a_walker_who_scrolled_stays_put_until_today_opens_again() {
    let held = Visit::default().home().scrolled(Scroll::Hold);
    assert_eq!(held.landing().step(Some(STEP)), None);
    assert_eq!(
        held.scrolled(Scroll::Follow).landing().step(Some(STEP)),
        Some(STEP)
    );
}

use basho::app::session::Appearance;
use basho::app::session::DarkAfterSunset;
use guide::clock::{ClockTime, Minutes, TripTime};

use crate::ui_check::Drawing;

const A_MINUTE: Minutes = Minutes::new(1);
const A_DAY: Minutes = Minutes::new(24 * 60);

struct Around {
    before: Appearance,
    after: Appearance,
}

impl Around {
    fn first_sunset(setting: DarkAfterSunset) -> Around {
        let trip = Drawing::BuiltIn.itinerary();
        let day = trip.days().first().unwrap();
        let set = day.sunset().unwrap().time().unwrap();
        let at = |time| TripTime::on_date_at(trip.head().zone(), day.date(), time);
        Around {
            before: Appearance::chosen(setting, Some(day), at(set.later_by(-A_MINUTE))),
            after: Appearance::chosen(setting, Some(day), at(set.later_by(A_MINUTE))),
        }
    }
}

#[test]
fn the_screen_turns_dark_at_sunset_when_the_setting_is_on() {
    let around = Around::first_sunset(DarkAfterSunset::On);
    assert_eq!(around.before, Appearance::Light);
    assert_eq!(around.after, Appearance::Dark);
}

#[test]
fn the_screen_stays_light_all_day_when_the_setting_is_off() {
    let around = Around::first_sunset(DarkAfterSunset::default());
    assert_eq!(around.before, Appearance::Light);
    assert_eq!(around.after, Appearance::Light);
}

#[test]
fn the_screen_stays_light_on_a_date_that_is_not_the_day() {
    let trip = Drawing::BuiltIn.itinerary();
    let day = trip.days().first().unwrap();
    let late = TripTime::on_date_at(trip.head().zone(), day.date(), ClockTime::at_hour(23));
    let next = late.later_by(A_DAY);
    assert_eq!(
        Appearance::chosen(DarkAfterSunset::On, Some(day), late),
        Appearance::Dark
    );
    assert_eq!(
        Appearance::chosen(DarkAfterSunset::On, Some(day), next),
        Appearance::Light
    );
}

use guide::clock::{Sunset, TripTime};
use guide::itinerary::Day;

use crate::app::session::DarkAfterSunset;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appearance {
    #[strum(serialize = "theme-light")]
    Light,
    #[strum(serialize = "theme-dark")]
    Dark,
}

impl Appearance {
    #[must_use]
    pub fn chosen(setting: DarkAfterSunset, day: Option<&Day>, now: TripTime) -> Appearance {
        let sunset = day
            .filter(|day| day.date() == now.trip_date())
            .and_then(Day::sunset)
            .and_then(Sunset::time);
        match (setting, sunset) {
            (DarkAfterSunset::On, Some(set)) if now.time() >= set => Appearance::Dark,
            _ => Appearance::Light,
        }
    }
}

impl guide::classed::Classed for Appearance {}

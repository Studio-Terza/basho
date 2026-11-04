use guide::clock::{ClockTime, Sunset, TripTime};
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
        Self::after(setting, sunset, now.time())
    }

    fn after(setting: DarkAfterSunset, sunset: Option<ClockTime>, now: ClockTime) -> Appearance {
        match setting {
            DarkAfterSunset::Off => Appearance::Light,
            DarkAfterSunset::On => sunset
                .filter(|set| now >= *set)
                .map_or(Appearance::Light, |_| Appearance::Dark),
        }
    }
}

impl guide::classed::Classed for Appearance {}

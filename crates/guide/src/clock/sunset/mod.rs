mod days;

use std::iter::successors;

use chrono::NaiveDate;

use crate::clock::{CalendarDate, ClockTime, Minutes, TripZone};
use crate::geo::{Coordinate, Degrees};
use days::Days;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sunset {
    date: CalendarDate,
    at: Coordinate,
    zone: TripZone,
}

impl Sunset {
    const J2000_SINCE_EPOCH: Days = Days::new(10_957.0);
    const J2000: Days = Days::new(2_451_545.0);
    const UNIX_EPOCH: Days = Days::new(2_440_587.5);
    const LEAP_CORRECTION: Days = Days::new(0.0008);
    const TRANSIT_ANOMALY: Days = Days::new(0.0053);
    const TRANSIT_ECLIPTIC: Days = Days::new(0.0069);
    const DAY: Minutes = Minutes::new(1_440);
    const FULL_TURN: Degrees = Degrees::new(360.0);
    const HALF_TURN: Degrees = Degrees::new(180.0);
    const MEAN_ANOMALY_AT_J2000: Degrees = Degrees::new(357.5291);
    const MEAN_ANOMALY_PER_DAY: Degrees = Degrees::new(0.985_600_28);
    const CENTRE: [Degrees; 3] = [
        Degrees::new(1.9148),
        Degrees::new(0.0200),
        Degrees::new(0.0003),
    ];
    const PERIHELION: Degrees = Degrees::new(102.9372);
    const AXIAL_TILT: Degrees = Degrees::new(23.4397);
    const HORIZON: Degrees = Degrees::new(-0.833);

    #[must_use]
    pub fn new(date: CalendarDate, at: Coordinate, zone: TripZone) -> Sunset {
        Sunset { date, at, zone }
    }

    #[must_use]
    pub fn time(self) -> Option<ClockTime> {
        let turn = Self::FULL_TURN.degrees();
        let since_epoch = f64::from(NaiveDate::from(self.date).to_epoch_days());
        let day = since_epoch - Self::J2000_SINCE_EPOCH.days() + Self::LEAP_CORRECTION.days()
            - self.at.lon().angle().degrees() / turn;
        let anomaly = (Self::MEAN_ANOMALY_AT_J2000.degrees()
            + Self::MEAN_ANOMALY_PER_DAY.degrees() * day)
            .rem_euclid(turn)
            .to_radians();
        let centre = Self::CENTRE
            .iter()
            .zip(successors(Some(1.0), |harmonic| Some(harmonic + 1.0)))
            .map(|(weight, harmonic)| weight.degrees() * (anomaly * harmonic).sin())
            .sum::<f64>();
        let ecliptic = (anomaly.to_degrees()
            + centre
            + Self::HALF_TURN.degrees()
            + Self::PERIHELION.degrees())
        .rem_euclid(turn)
        .to_radians();
        let transit = Self::J2000.days() + day + Self::TRANSIT_ANOMALY.days() * anomaly.sin()
            - Self::TRANSIT_ECLIPTIC.days() * (ecliptic + ecliptic).sin();
        let declination = (ecliptic.sin() * Self::AXIAL_TILT.degrees().to_radians().sin()).asin();
        let latitude = self.at.lat().angle().degrees().to_radians();
        let hour_angle = (Self::HORIZON.degrees().to_radians().sin()
            - latitude.sin() * declination.sin())
            / (latitude.cos() * declination.cos());
        let setting = transit + hour_angle.acos().to_degrees() / turn;
        let into_day = (setting - Self::UNIX_EPOCH.days() - since_epoch) * Self::DAY.amount()
            + self.zone.ahead_of_utc().amount();
        (-1.0..=1.0)
            .contains(&hour_angle)
            .then(|| Minutes::rounded(into_day))
            .flatten()
            .filter(|minutes| u16::try_from(minutes.count()).is_ok())
            .map(ClockTime::at_minutes)
    }
}

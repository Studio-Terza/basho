use std::time::{Duration, SystemTime};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeDelta, Timelike, Utc};

use crate::clock::calendar_date::CalendarDate;
use crate::clock::clock_time::ClockTime;
use crate::clock::day_tense::DayTense;
use crate::clock::minutes::Minutes;
use crate::clock::trip_zone::TripZone;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TripTime(DateTime<FixedOffset>);

impl TripTime {
    const DAY_ENDS_AFTER_MIDNIGHT: TimeDelta = TimeDelta::hours(4);

    #[must_use]
    pub fn on_date_at(zone: TripZone, date: CalendarDate, time: ClockTime) -> TripTime {
        let local = NaiveDateTime::new(NaiveDate::from(date), time.naive());
        TripTime::at_utc(local.and_utc() - FixedOffset::from(zone), zone)
    }

    #[must_use]
    pub fn at_instant(instant: SystemTime, zone: TripZone) -> TripTime {
        TripTime::at_utc(DateTime::from(instant), zone)
    }

    #[must_use]
    pub fn earlier_by(self, gap: Duration) -> TripTime {
        TripTime::at_instant(SystemTime::from(self) - gap, self.zone())
    }

    #[must_use]
    pub fn zone(self) -> TripZone {
        TripZone::from(*self.0.offset())
    }

    #[must_use]
    pub fn time(self) -> ClockTime {
        ClockTime::from(self.0.time())
    }

    #[must_use]
    pub fn minutes_since(self, earlier: TripTime) -> Minutes {
        Minutes::new(self.0.signed_duration_since(earlier.0).num_minutes())
    }

    #[must_use]
    pub fn later_by(self, minutes: Minutes) -> TripTime {
        TripTime(self.0 + TimeDelta::minutes(minutes.count()))
    }

    #[must_use]
    pub fn trip_date(self) -> CalendarDate {
        CalendarDate::from((self.0 - Self::DAY_ENDS_AFTER_MIDNIGHT).date_naive())
    }

    #[must_use]
    pub fn minutes_into(self, date: CalendarDate) -> Minutes {
        self.minutes_since(TripTime::on_date_at(
            self.zone(),
            date,
            ClockTime::at_hour(0),
        ))
    }

    #[must_use]
    pub fn tense_of(self, date: CalendarDate) -> DayTense {
        match self.trip_date().cmp(&date) {
            std::cmp::Ordering::Less => DayTense::Ahead,
            std::cmp::Ordering::Equal => DayTense::Today,
            std::cmp::Ordering::Greater => DayTense::Past,
        }
    }

    pub(super) fn naive(self) -> NaiveDateTime {
        self.0.naive_local()
    }

    fn at_utc(instant: DateTime<Utc>, zone: TripZone) -> TripTime {
        let whole_minute = instant
            .with_second(0)
            .and_then(|start| start.with_nanosecond(0))
            .unwrap_or(instant);
        TripTime(whole_minute.with_timezone(&FixedOffset::from(zone)))
    }
}

impl From<TripTime> for CalendarDate {
    fn from(time: TripTime) -> CalendarDate {
        CalendarDate::from(time.0.date_naive())
    }
}

impl From<TripTime> for SystemTime {
    fn from(time: TripTime) -> SystemTime {
        SystemTime::from(time.0)
    }
}

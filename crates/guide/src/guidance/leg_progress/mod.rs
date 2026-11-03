mod progress_error;
use crate::words::Written;
use newtype::Counter;
use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::clock::{ClockTime, Minutes, TripTime};
use crate::day_number::DayNumber;
use crate::end::End;
use crate::geo::{Coordinate, Degrees, Distance};
use crate::guidance::nearness::Nearness;
use crate::guidance::tracking::{Placing, Stage, Tracking};
use crate::itinerary::Day;
use crate::leg::{LegIndex, Mode};
use progress_error::ProgressError;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LegProgress {
    day: Option<DayNumber>,
    leg: LegIndex,
    along: Distance,
    furthest: Distance,
    offset: Minutes,
    at: Option<ClockTime>,
    seen: Option<Coordinate>,
    tracking: Tracking,
}

impl LegProgress {
    #[must_use]
    pub fn leg(self) -> LegIndex {
        self.leg
    }

    #[must_use]
    pub fn along(self) -> Distance {
        self.along
    }

    const NO_DAY: &str = "none";

    #[must_use]
    pub fn confirmed(day: DayNumber, leg: LegIndex, offset: Minutes) -> LegProgress {
        LegProgress {
            day: Some(day),
            leg,
            along: Distance::default(),
            furthest: Distance::default(),
            offset,
            at: None,
            seen: None,
            tracking: Tracking::Automatic,
        }
    }

    #[must_use]
    pub fn tracking(self) -> Tracking {
        self.tracking
    }

    #[must_use]
    pub fn tracked(self, tracking: Tracking) -> LegProgress {
        LegProgress { tracking, ..self }
    }

    #[must_use]
    pub fn pinned_at(day: &Day, placing: Placing, now: TripTime, baseline: Placing) -> LegProgress {
        let offset = day.leg(placing.on_leg()).map_or(Minutes::ZERO, |leg| {
            placing.lateness(leg, now.minutes_into(now.trip_date()))
        });
        let pinned = LegProgress::confirmed(day.number(), placing.on_leg(), offset)
            .tracked(Tracking::pinned(placing, baseline));
        match (placing.stage(), day.leg(placing.on_leg())) {
            (Stage::Finished, Some(leg)) => pinned.placed(leg.length(), now),
            _ => pinned,
        }
    }

    #[must_use]
    pub fn dawn(day: &Day) -> LegProgress {
        LegProgress::confirmed(day.number(), LegIndex::default(), Minutes::ZERO)
    }

    #[must_use]
    pub fn placed(self, along: Distance, at: TripTime) -> LegProgress {
        LegProgress {
            along,
            furthest: along,
            at: Some(at.time()),
            ..self
        }
    }

    #[must_use]
    pub fn reaching(self, furthest: Distance) -> LegProgress {
        LegProgress {
            furthest: furthest.max(self.along),
            ..self
        }
    }

    #[must_use]
    pub fn reached(self) -> RangeInclusive<Distance> {
        self.along..=self.furthest
    }

    #[must_use]
    pub fn for_day(self, day: DayNumber) -> Option<LegProgress> {
        (self.day == Some(day)).then_some(self)
    }

    #[must_use]
    pub fn seen_at(self, here: Coordinate) -> LegProgress {
        LegProgress {
            seen: Some(here),
            ..self
        }
    }

    #[must_use]
    pub fn moved_to(self, here: Coordinate) -> Option<Distance> {
        self.seen.map(|seen| seen.distance_to(here))
    }

    #[must_use]
    pub fn offset(self) -> Minutes {
        self.offset
    }

    #[must_use]
    pub fn minutes_before(self, day: &Day, now: TripTime) -> Minutes {
        let seen = self.at.map_or_else(
            || {
                day.leg(self.leg).map_or(Minutes::ZERO, |leg| {
                    leg.time(End::Start).since_midnight()
                        + leg.planned_after(self.along)
                        + self.offset
                })
            },
            ClockTime::since_midnight,
        );
        (now.minutes_into(day.date()) - seen).max(Minutes::ZERO)
    }

    #[must_use]
    pub fn resumed(self, day: &Day) -> LegIndex {
        day.leg(self.leg)
            .filter(|leg| self.along >= leg.length() - Nearness::GpsError.distance())
            .and_then(|_| day.leg(self.leg.incremented()))
            .filter(|next| next.mode() == Mode::Walk)
            .map_or(self.leg, |_| self.leg.incremented())
    }

    #[must_use]
    pub fn arrived(self, day: &Day) -> bool {
        let last = day.legs().len().checked_sub(1).map(LegIndex::new);
        self.for_day(day.number()).is_some_and(|kept| {
            Some(kept.leg) == last
                && day
                    .leg(kept.leg)
                    .is_some_and(|leg| kept.along >= leg.length() - Nearness::GpsError.distance())
        })
    }
}

impl Eq for LegProgress {}

impl LegProgress {
    fn offset_word<'text>(rest: &[&'text str]) -> Option<&'text str> {
        match rest {
            ["offset" | "behind", offset] | ["behind", _, "started", offset] => Some(*offset),
            _ => Self::word_after(rest, "offset"),
        }
    }

    fn word_after<'text>(rest: &[&'text str], label: &str) -> Option<&'text str> {
        rest.iter()
            .zip(rest.iter().skip(1))
            .find(|(word, _)| **word == label)
            .map(|(_, next)| *next)
    }

    fn along_word(rest: &[&str], label: &str) -> Distance {
        Self::word_after(rest, label)
            .and_then(|along| along.parse().ok())
            .map_or(Distance::default(), Distance::new)
    }

    fn seen_word(rest: &[&str]) -> Option<Coordinate> {
        rest.iter()
            .position(|word| *word == "from")
            .and_then(|from| match rest.get(from..)? {
                [_, lat, lon, ..] => Some(Coordinate::new(
                    Degrees::new(lat.parse().ok()?),
                    Degrees::new(lon.parse().ok()?),
                )),
                _ => None,
            })
    }
}

impl fmt::Display for LegProgress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let day = self
            .day
            .map_or_else(|| Self::NO_DAY.to_owned(), |day| day.to_string());
        write!(
            formatter,
            "day {day} leg {} along {:.0} furthest {:.0} offset {}",
            self.leg,
            self.along.meters(),
            self.furthest.meters(),
            self.offset.count()
        )?;
        self.at.map_or(Ok(()), |at| {
            write!(formatter, " at {}", at.since_midnight().count())
        })?;
        self.seen.map_or(Ok(()), |seen| {
            write!(
                formatter,
                " from {:.6} {:.6}",
                seen.lat().angle().degrees(),
                seen.lon().angle().degrees()
            )
        })?;
        write!(formatter, "{}", self.tracking)
    }
}

impl FromStr for LegProgress {
    type Err = ProgressError;

    fn from_str(text: &str) -> Result<LegProgress, ProgressError> {
        let words: Vec<&str> = text.split_whitespace().collect();
        match words.as_slice() {
            ["day", day, "leg", leg, rest @ ..] => Self::offset_word(rest).and_then(|offset| {
                Some(LegProgress {
                    day: Some(DayNumber::new(day.parse().ok()?)),
                    leg: LegIndex::new(leg.parse::<usize>().ok()?.checked_sub(1)?),
                    along: Self::along_word(rest, "along"),
                    furthest: Self::along_word(rest, "furthest")
                        .max(Self::along_word(rest, "along")),
                    offset: Minutes::new(offset.parse().ok()?),
                    at: Self::word_after(rest, "at")
                        .map(str::parse)
                        .and_then(Result::ok)
                        .map(Minutes::new)
                        .map(ClockTime::at_minutes),
                    seen: Self::seen_word(rest),
                    tracking: rest.join(" ").parse().unwrap_or_default(),
                })
            }),
            _ => None,
        }
        .ok_or_else(|| ProgressError::Unreadable(Written::new(text.to_owned())))
    }
}

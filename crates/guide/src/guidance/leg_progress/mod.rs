mod progress_error;
use crate::words::{Label, Written};
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

struct ProgressWords<'text>(Vec<&'text str>);

impl ProgressWords<'_> {
    const HEAD: usize = 4;

    fn after<Value: FromStr>(&self, label: Label) -> Option<Value> {
        self.0
            .iter()
            .zip(self.0.iter().skip(1))
            .find(|(word, _)| **word == AsRef::<str>::as_ref(&label))
            .and_then(|(_, next)| next.parse().ok())
    }

    fn offset(&self) -> Option<Minutes> {
        match self.0.as_slice() {
            ["day", _, "leg", _, "offset" | "behind", offset]
            | ["day", _, "leg", _, "behind", _, "started", offset] => {
                offset.parse().ok().map(Minutes::new)
            }
            _ => self.after(Label::new("offset")).map(Minutes::new),
        }
    }

    fn seen(&self) -> Option<Coordinate> {
        self.0
            .iter()
            .position(|word| *word == "from")
            .and_then(|from| match self.0.get(from..)? {
                [_, lat, lon, ..] => Some(Coordinate::new(
                    Degrees::new(lat.parse().ok()?),
                    Degrees::new(lon.parse().ok()?),
                )),
                _ => None,
            })
    }

    fn progress(&self) -> Option<LegProgress> {
        <[&str; ProgressWords::HEAD]>::try_from(self.0.get(..ProgressWords::HEAD)?)
            .ok()
            .filter(|[day_label, _, leg_label, _]| (*day_label, *leg_label) == ("day", "leg"))
            .and_then(|[_, day, _, leg]| {
                let along =
                    |label: Label| self.after(label).map_or(Distance::default(), Distance::new);
                Some(LegProgress {
                    day: Some(DayNumber::new(day.parse().ok()?)),
                    leg: LegIndex::new(leg.parse::<usize>().ok()?.checked_sub(1)?),
                    along: along(Label::new("along")),
                    furthest: along(Label::new("furthest")).max(along(Label::new("along"))),
                    offset: self.offset()?,
                    at: self
                        .after(Label::new("at"))
                        .map(Minutes::new)
                        .map(ClockTime::at_minutes),
                    seen: self.seen(),
                    tracking: self.0[ProgressWords::HEAD..]
                        .join(" ")
                        .parse()
                        .unwrap_or_default(),
                })
            })
    }
}

impl FromStr for LegProgress {
    type Err = ProgressError;

    fn from_str(text: &str) -> Result<LegProgress, ProgressError> {
        ProgressWords(text.split_whitespace().collect())
            .progress()
            .ok_or_else(|| ProgressError::Unreadable(Written::new(text.to_owned())))
    }
}

mod mountain_range;
mod peak;
mod river;
mod sea;
mod star_key;

use std::collections::BTreeSet;

use crate::StarPoint;
use crate::day_number::DayNumber;
use crate::geo::Coordinate;
use serde::Deserialize;

use crate::end::End;
use crate::geo::Distance;
use crate::itinerary::{Day, Finding, Itinerary, Problem};
use crate::leg::{LegIndex, Mode};
use crate::words::StarName;

pub use mountain_range::MountainRange;
pub use peak::Peak;
pub use river::River;
pub use sea::Sea;
use star_key::StarKey;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Landmarks {
    #[serde(default)]
    peaks: Vec<Peak>,
    #[serde(default)]
    ranges: Vec<MountainRange>,
    #[serde(default)]
    seas: Vec<Sea>,
    #[serde(default)]
    rivers: Vec<River>,
    #[serde(default)]
    stars: Vec<StarPoint>,
}

impl Landmarks {
    const PASSED: Distance = Distance::new(100.0);

    #[must_use]
    pub fn peaks(&self) -> &[Peak] {
        &self.peaks
    }

    #[must_use]
    pub fn ranges(&self) -> &[MountainRange] {
        &self.ranges
    }

    #[must_use]
    pub fn seas(&self) -> &[Sea] {
        &self.seas
    }

    #[must_use]
    pub fn rivers(&self) -> &[River] {
        &self.rivers
    }

    #[must_use]
    pub fn star_points(&self) -> &[StarPoint] {
        &self.stars
    }

    #[must_use]
    pub fn findings(&self, itinerary: &Itinerary) -> Vec<Finding> {
        let known: BTreeSet<StarKey<'_>> = self.stars.iter().map(StarKey::from).collect();
        itinerary
            .days()
            .iter()
            .flat_map(|day| Self::unknown_claims(&known, day))
            .chain(self.stars.iter().filter_map(|point| {
                itinerary
                    .day_number(DayNumber::from(point))
                    .and_then(|day| Self::unclaimed(point, day))
            }))
            .chain(self.repeated_names())
            .collect()
    }

    fn repeated_names(&self) -> impl Iterator<Item = Finding> + '_ {
        self.stars
            .iter()
            .scan(BTreeSet::new(), |seen, point| {
                Some(seen.replace(StarKey::from(point)))
            })
            .flatten()
            .map(|earlier| {
                Finding::on_day(
                    earlier.day(),
                    Problem::DuplicateStarName {
                        name: earlier.name().clone(),
                    },
                )
            })
    }

    fn unknown_claims<'day>(
        known: &'day BTreeSet<StarKey<'_>>,
        day: &'day Day,
    ) -> impl Iterator<Item = Finding> + 'day {
        day.goals().iter().flat_map(move |goal| {
            goal.purpose()
                .claims()
                .iter()
                .filter_map(move |name| Self::unknown(known, day, goal.leg(End::Finish), name))
        })
    }

    fn unknown(
        known: &BTreeSet<StarKey<'_>>,
        day: &Day,
        leg: LegIndex,
        name: &StarName,
    ) -> Option<Finding> {
        match known.get(&StarKey::new(day.number(), name)) {
            Some(_) => None,
            None => Some(Finding::on_leg(
                day.number(),
                leg,
                Problem::UnknownStarPoint { name: name.clone() },
            )),
        }
    }

    fn unclaimed(point: &StarPoint, day: &Day) -> Option<Finding> {
        day.legs()
            .iter()
            .filter(|leg| leg.mode() == Mode::Walk)
            .filter_map(|leg| leg.line().fix(Coordinate::from(point)))
            .map(|fix| Distance::from(fix.off_route()))
            .min_by(|left, right| left.meters().total_cmp(&right.meters()))
            .filter(|off| *off <= Self::PASSED)
            .filter(|_| !day.claims(point.short()))
            .map(|off| {
                Finding::on_day(
                    day.number(),
                    Problem::UnclaimedStarPoint {
                        name: point.short().to_owned(),
                        off,
                    },
                )
            })
    }
}

mod mountain_range;
mod peak;
mod river;
mod sea;
mod star_point;
use crate::geo::Coordinate;
use crate::itinerary::DayNumber;
use crate::words::StarName;
use serde::Deserialize;

use crate::end::End;
use crate::geo::Distance;
use crate::itinerary::{Day, Finding, Itinerary, Problem};
use crate::leg::Mode;

pub use mountain_range::MountainRange;
pub use peak::{Elevation, Peak};
pub use river::River;
pub use sea::Sea;
pub use star_point::StarPoint;

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
        itinerary
            .days()
            .iter()
            .flat_map(|day| self.unknown_claims(day))
            .chain(self.stars.iter().filter_map(|point| {
                itinerary
                    .day_number(DayNumber::from(point))
                    .and_then(|day| Self::unclaimed(point, day))
            }))
            .collect()
    }

    fn unknown_claims<'day>(&'day self, day: &'day Day) -> impl Iterator<Item = Finding> + 'day {
        day.goals().iter().flat_map(move |goal| {
            goal.purpose()
                .claims()
                .iter()
                .filter(move |name| !self.knows(day, name))
                .map(move |name| {
                    Finding::on_leg(
                        day.number(),
                        goal.leg(End::Finish),
                        Problem::UnknownStarPoint { name: name.clone() },
                    )
                })
        })
    }

    fn knows(&self, day: &Day, name: &StarName) -> bool {
        self.stars
            .iter()
            .any(|point| DayNumber::from(point) == day.number() && point.short() == name)
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

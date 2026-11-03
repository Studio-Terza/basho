use crate::content::{Content, ContentList};
mod catch_sight;
pub mod feature;
pub mod leg_catch;
use crate::readout::recovery::catch_words::Readings;
use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::day_number::DayNumber;
use crate::geo::{Coordinate, Degrees, Distance};
use crate::leg::LegIndex;
use crate::readout::pointer::Pointer;
use crate::words::Spelling;
use feature::Feature;
use leg_catch::LegCatch;

pub use catch_sight::CatchSight;

#[derive(derive_more::AsRef, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DayCatch {
    #[expect(dead_code, reason = "import date, kept only so the file loads")]
    checked: Option<IgnoredAny>,
    #[as_ref]
    day: DayNumber,
    features: Vec<Feature>,
    legs: Vec<LegCatch>,
}

impl<'catch> From<&'catch DayCatch> for ContentList<'catch> {
    fn from(catch: &'catch DayCatch) -> ContentList<'catch> {
        catch
            .legs
            .iter()
            .flat_map(LegCatch::notes)
            .filter_map(|note| note.ste())
            .collect()
    }
}

impl DayCatch {
    pub const PICKED_LENGTH: usize = 2;
    const TOWARD_ROUTE: Degrees = Degrees::new(90.0);
    const ON_THE_ROUTE: Distance = Distance::new(35.0);

    pub fn spot_names(&self) -> impl Iterator<Item = Spelling> + '_ {
        self.features.iter().flat_map(Feature::spot_names)
    }

    #[must_use]
    pub fn toward(
        &self,
        here: Coordinate,
        leg: LegIndex,
        route: Option<Pointer>,
    ) -> Vec<CatchSight<'_>> {
        let leads = |sight: &CatchSight<'_>| {
            route
                .filter(|route| route.distance() >= Self::ON_THE_ROUTE)
                .is_none_or(|route| {
                    sight.pointer().bearing().gap_to(route.bearing())
                        <= Self::TOWARD_ROUTE.degrees()
                })
        };
        let picked: Vec<CatchSight<'_>> = self
            .legs
            .iter()
            .flat_map(|catch| catch.notes_for(leg))
            .filter_map(|note| {
                self.features
                    .iter()
                    .find(|feature| feature.feature_id() == note.feature())
                    .and_then(|feature| {
                        Self::sighted(feature, note.ste().map(Content::shown), here)
                    })
            })
            .filter(leads)
            .collect();
        let pool: Vec<CatchSight<'_>> = if picked.is_empty() {
            self.features
                .iter()
                .filter_map(|feature| Self::sighted(feature, None, here))
                .filter(leads)
                .collect()
        } else {
            picked
        };
        pool.into_iter()
            .fold(Vec::new(), |mut nearest: Vec<CatchSight<'_>>, sight| {
                let place = nearest.partition_point(|kept| {
                    kept.pointer().distance() <= sight.pointer().distance()
                });
                nearest.insert(place, sight);
                nearest
            })
            .into_iter()
            .take(Self::PICKED_LENGTH)
            .collect()
    }

    fn sighted<'catch>(
        feature: &'catch Feature,
        note: Option<&'catch crate::ruby_text::RubyText>,
        here: Coordinate,
    ) -> Option<CatchSight<'catch>> {
        feature.nearest_to(here).map(|nearest| {
            CatchSight::new(
                feature.as_ref(),
                feature.reference(),
                note,
                Pointer::between(here, nearest),
            )
        })
    }
}

impl DayCatch {
    #[must_use]
    pub fn read_with(self, readings: &Readings) -> DayCatch {
        DayCatch {
            features: self
                .features
                .into_iter()
                .map(|feature| feature.read_again(readings))
                .collect(),
            ..self
        }
    }
}

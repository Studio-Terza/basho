mod findings;
mod kana_file;
mod once;
use crate::words::Label;
use findings::Findings;
pub use kana_file::KanaFile;
use once::Once;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use std::thread::{Scope, ScopedJoinHandle};

use serde::Deserialize;

use crate::itinerary::{Arc, Day, Finding, Itinerary, LegLines, Problem};

use crate::KnownNames;
use crate::NonEmpty;
use crate::TripSource;
use crate::day_number::DayNumber;
use crate::landmark::Landmarks;
use crate::overview::Overview;
use crate::place::Place;
use crate::place_catalog::PlaceCatalog;
use crate::readout::DayCatch;
use crate::trip::{Trip, TripError};
use crate::trip_head::TripHead;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TripFile {
    trip: TripHead,
    arcs: Vec<Arc>,
    days: Vec<Day>,
    #[serde(default)]
    places: Vec<Place>,
    #[serde(default)]
    landmarks: Landmarks,
    #[serde(default)]
    routes: Vec<LegLines>,
    #[serde(default)]
    kana: KanaFile,
    #[serde(default)]
    catches: Vec<DayCatch>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct TripPart {
    #[serde(default)]
    trip: Once<TripHead>,
    #[serde(default)]
    arcs: Vec<Arc>,
    #[serde(default)]
    days: Vec<Day>,
    #[serde(default)]
    places: Vec<Place>,
    #[serde(default)]
    landmarks: Once<Landmarks>,
    #[serde(default)]
    routes: Vec<LegLines>,
    #[serde(default)]
    kana: Once<KanaFile>,
    #[serde(default)]
    catches: Vec<DayCatch>,
}

impl TripPart {
    fn started<'scope>(
        scope: &'scope Scope<'scope, '_>,
        stretch: TripSource<'scope>,
    ) -> ScopedJoinHandle<'scope, Option<TripPart>> {
        scope.spawn(move || toml::from_str(stretch.as_ref()).ok())
    }

    fn joined(self, next: TripPart) -> Option<TripPart> {
        Some(TripPart {
            trip: self.trip.joined(next.trip)?,
            arcs: self.arcs.into_iter().chain(next.arcs).collect(),
            days: self.days.into_iter().chain(next.days).collect(),
            places: self.places.into_iter().chain(next.places).collect(),
            landmarks: self.landmarks.joined(next.landmarks)?,
            routes: self.routes.into_iter().chain(next.routes).collect(),
            kana: self.kana.joined(next.kana)?,
            catches: self.catches.into_iter().chain(next.catches).collect(),
        })
    }

    fn whole(self) -> Option<TripFile> {
        let arcs = NonEmpty::new(self.arcs)?;
        let days = NonEmpty::new(self.days)?;
        Some(TripFile {
            trip: self.trip.given()?,
            arcs: Vec::from(arcs),
            days: Vec::from(days),
            places: self.places,
            landmarks: self.landmarks.given().unwrap_or_default(),
            routes: self.routes,
            kana: self.kana.given().unwrap_or_default(),
            catches: self.catches,
        })
    }
}

impl FromStr for TripFile {
    type Err = TripError;

    fn from_str(text: &str) -> Result<TripFile, TripError> {
        match TripFile::in_stretches(TripSource::new(text)) {
            Some(file) => Ok(file),
            None => toml::from_str::<TripFile>(text).map_err(TripError::Toml),
        }
    }
}

impl TripFile {
    pub fn in_stretches(source: TripSource<'_>) -> Option<TripFile> {
        let stretches = source.stretches();
        std::thread::scope(|scope| {
            stretches
                .iter()
                .map(|stretch| TripPart::started(scope, *stretch))
                .collect::<Vec<_>>()
                .into_iter()
                .map(|worker| worker.join().ok().flatten())
                .collect::<Option<Vec<TripPart>>>()
        })?
        .into_iter()
        .try_fold(TripPart::default(), TripPart::joined)?
        .whole()
    }

    pub fn trip(self) -> Result<Trip, TripError> {
        let known: Vec<DayNumber> = self.days.iter().map(Day::number).collect();
        let in_arcs = self.arcs.iter().flat_map(|arc| arc.day_numbers().iter());
        let strays: Vec<Finding> = [
            Self::strays("arcs", in_arcs.copied(), &known),
            Self::strays("routes", self.routes.iter().map(DayNumber::from), &known),
            Self::strays("catches", self.catches.iter().map(DayNumber::from), &known),
        ]
        .concat();
        let stray_lines: Vec<Finding> = self
            .routes
            .iter()
            .flat_map(|lines| {
                self.days
                    .iter()
                    .filter(move |day| day.number() == DayNumber::from(lines))
                    .flat_map(|day| lines.beyond(day))
                    .map(move |leg| {
                        Finding::on_leg(DayNumber::from(lines), leg, Problem::StrayRoute)
                    })
            })
            .collect();
        let readings = self.kana.sourced();
        let mut routes: BTreeMap<DayNumber, LegLines> = self
            .routes
            .into_iter()
            .map(|lines| (DayNumber::from(&lines), lines))
            .collect();
        let mut catches: BTreeMap<DayNumber, DayCatch> = self
            .catches
            .into_iter()
            .map(|catch| (DayNumber::from(&catch), catch.read_with(&readings)))
            .collect();
        let zone = self.trip.zone();
        let days = self
            .days
            .into_iter()
            .map(|day| {
                let number = day.number();
                day.assembled(
                    zone,
                    &routes.remove(&number).unwrap_or_default(),
                    catches.remove(&number),
                )
            })
            .collect();
        let itinerary = Itinerary::new(self.trip, self.arcs, days);
        let places = PlaceCatalog::new(self.places);
        let overview = Overview::from(self.landmarks);
        let named = KnownNames::new(&itinerary, places.places(), overview.landmarks()).in_words(
            Trip::contents(&itinerary, &places, &overview)
                .iter()
                .copied(),
        );
        let findings: Findings = strays
            .into_iter()
            .chain(stray_lines)
            .chain(Vec::<Finding>::from(&itinerary))
            .chain(overview.landmarks().findings(&itinerary))
            .chain(places.findings(&itinerary))
            .chain(named)
            .collect();
        findings.into_result(Trip::new(itinerary, places, overview))
    }

    fn strays(
        section: &'static str,
        days: impl Iterator<Item = DayNumber>,
        known: &[DayNumber],
    ) -> Vec<Finding> {
        let days: Vec<DayNumber> = days.collect();
        let first_at: BTreeMap<DayNumber, usize> = days
            .iter()
            .copied()
            .enumerate()
            .rev()
            .map(|(at, day)| (day, at))
            .collect();
        let accepted: BTreeSet<usize> = known
            .iter()
            .filter_map(|day| first_at.get(day))
            .copied()
            .collect();
        let every: BTreeSet<usize> = (0..days.len()).collect();
        every
            .difference(&accepted)
            .filter_map(|at| days.get(*at))
            .map(|day| {
                Finding::on_day(
                    *day,
                    Problem::StraySection {
                        section: Label::new(section),
                    },
                )
            })
            .collect()
    }
}

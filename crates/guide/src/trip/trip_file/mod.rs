mod kana_file;
use crate::words::Label;
pub use kana_file::KanaFile;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use std::thread::{Scope, ScopedJoinHandle};

use serde::Deserialize;

use crate::itinerary::{Arc, Day, Finding, Itinerary, LegLines, Problem};

use crate::KnownNames;
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
    trip: Option<TripHead>,
    #[serde(default)]
    arcs: Vec<Arc>,
    #[serde(default)]
    days: Vec<Day>,
    #[serde(default)]
    places: Vec<Place>,
    landmarks: Option<Landmarks>,
    #[serde(default)]
    routes: Vec<LegLines>,
    kana: Option<KanaFile>,
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
        Some(self)
            .filter(|part| part.trip.as_ref().zip(next.trip.as_ref()).is_none())
            .filter(|part| {
                part.landmarks
                    .as_ref()
                    .zip(next.landmarks.as_ref())
                    .is_none()
            })
            .filter(|part| part.kana.as_ref().zip(next.kana.as_ref()).is_none())
            .map(|part| TripPart {
                trip: part.trip.or(next.trip),
                arcs: part.arcs.into_iter().chain(next.arcs).collect(),
                days: part.days.into_iter().chain(next.days).collect(),
                places: part.places.into_iter().chain(next.places).collect(),
                landmarks: part.landmarks.or(next.landmarks),
                routes: part.routes.into_iter().chain(next.routes).collect(),
                kana: part.kana.or(next.kana),
                catches: part.catches.into_iter().chain(next.catches).collect(),
            })
    }

    fn whole(self) -> Option<TripFile> {
        Some(self)
            .filter(|part| part.arcs.first().zip(part.days.first()).is_some())
            .and_then(|part| {
                part.trip.map(|trip| TripFile {
                    trip,
                    arcs: part.arcs,
                    days: part.days,
                    places: part.places,
                    landmarks: part.landmarks.unwrap_or_default(),
                    routes: part.routes,
                    kana: part.kana.unwrap_or_default(),
                    catches: part.catches,
                })
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
        let findings: Vec<Finding> = strays
            .into_iter()
            .chain(stray_lines)
            .chain(Vec::<Finding>::from(&itinerary))
            .chain(overview.landmarks().findings(&itinerary))
            .chain(places.findings(&itinerary))
            .chain(named)
            .collect();
        Some(findings)
            .filter(|findings| findings.iter().next().is_some())
            .map_or_else(
                || Ok(Trip::new(itinerary, places, overview)),
                |findings| Err(TripError::Invalid(findings)),
            )
    }

    fn strays(
        section: &'static str,
        days: impl Iterator<Item = DayNumber>,
        known: &[DayNumber],
    ) -> Vec<Finding> {
        let mut seen = BTreeSet::new();
        days.filter(|day| {
            Some(*day)
                .filter(|day| known.contains(day))
                .is_none_or(|day| seen.replace(day).is_some())
        })
        .map(|day| {
            Finding::on_day(
                day,
                Problem::StraySection {
                    section: Label::new(section),
                },
            )
        })
        .collect()
    }
}

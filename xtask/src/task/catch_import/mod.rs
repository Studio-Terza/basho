pub mod catch_section;
pub mod day_file;
mod trip_day;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use guide::KnownNames;
use guide::words::{Label, Spelling};
use guide::{Trip, TripSource};

use crate::task::failure::Failure;
use crate::task::trip_text::TripText;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use catch_section::CatchSection;
use day_file::DayFile;
use trip_day::TripDay;

pub struct CatchImport;

impl CatchImport {
    const KANA: &str = "kana.json";
    const KANA_SECTION: &str = "kana";
    const OPENING: &str = "\n[kana";
    const JSON: &str = "json";
    const ARGUMENT_INDEX: usize = 2;

    fn kana_in(research: &Path, trip_text: TripSource<'_>) -> Result<DayFile, Failure> {
        match fs::read_to_string(research.join(Self::KANA)) {
            Ok(text) => text.parse::<DayFile>().map_err(Failure::Catch),
            Err(error) if error.kind() == ErrorKind::NotFound => Self::kana_kept(trip_text),
            Err(error) => Err(Self::failed(&research.join(Self::KANA), error)),
        }
    }

    pub fn kana_kept(trip_text: TripSource<'_>) -> Result<DayFile, Failure> {
        let mut table = toml::from_str::<toml::Table>(trip_text.as_ref())
            .map_err(|error| Failure::Shared(Box::new(error)))?;
        let kana = table
            .remove(Self::KANA_SECTION)
            .unwrap_or_else(|| toml::Value::Table(toml::Table::new()));
        serde_json::to_string(&kana)
            .map_err(Failure::Catch)?
            .parse::<DayFile>()
            .map_err(Failure::Catch)
    }

    fn imported(from: &Path, kana: &DayFile, names: &KnownNames) -> Result<DayFile, Failure> {
        let day = fs::read_to_string(from)
            .map_err(|error| Self::failed(from, error))?
            .parse::<DayFile>()
            .map_err(Failure::Catch)?;
        let readings = kana.sourced_over(day.feature_readings());
        Ok(day.annotated(&readings, names))
    }

    fn failed(path: &Path, error: std::io::Error) -> Failure {
        Failure::Io {
            site: Site::from(path),
            error,
        }
    }
}

impl CatchImport {
    pub fn import_catches(workspace: &Workspace) -> Result<(), Failure> {
        let research = env::args()
            .nth(Self::ARGUMENT_INDEX)
            .map(PathBuf::from)
            .ok_or_else(|| {
                Failure::Usage(Spelling::new(
                    "usage: cargo xtask catch-import <research/catch dir>".to_owned(),
                ))
            })?;
        let path = TripText::path_in(workspace);
        let text = fs::read_to_string(&path).map_err(|error| Self::failed(&path, error))?;
        let trip = text.parse::<Trip>()?;
        let kana = Self::kana_in(&research, TripSource::new(&text))?;
        let names = KnownNames::new(
            trip.itinerary(),
            trip.places().places(),
            trip.overview().landmarks(),
        );
        let days = trip
            .itinerary()
            .days()
            .iter()
            .map(|day| {
                Self::imported(
                    &TripDay::of_day(day.number()).file(&research, Self::JSON),
                    &kana,
                    &names,
                )
            })
            .collect::<Result<Vec<DayFile>, Failure>>()?;
        let section = CatchSection::new(kana, days)
            .text()
            .map_err(Failure::CatchToml)?;
        section
            .spliced_into(TripSource::new(&text), Label::new(Self::OPENING), None)
            .saved_if_valid(&path)?;
        println!("catch-import: wrote {}", path.display());
        Ok(())
    }
}

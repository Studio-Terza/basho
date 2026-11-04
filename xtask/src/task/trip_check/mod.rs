mod checked_trip;

use guide::TripSource;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use file_ports::{FileFailure, Files};
use newtype::Newtype;

use crate::task::failure::Failure;
use crate::task::flag::Flag;
use crate::task::ports::Host;
use crate::task::trip_text::TripText;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use checked_trip::CheckedTrip;

pub struct TripCheck;

impl TripCheck {
    pub const FOLDER: &str = "data/trips";
}

impl TripCheck {
    pub fn print_problems(
        workspace: &Workspace,
        mut flags: impl Iterator<Item = Flag>,
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        let path = flags.next().map_or_else(
            || TripText::path_in(workspace),
            |flag| PathBuf::from(flag.into_inner()),
        );
        Self::check_each(&[path], host)
    }

    pub fn every_file_in(
        folder: &Path,
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        let paths = host
            .files()
            .entries(folder)
            .map_err(|failure| Failure::Io {
                site: Site::from(folder),
                error: io::Error::from(failure),
            })?;
        Self::check_each(&paths, host)
    }

    fn check_each(
        paths: &[PathBuf],
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        let texts = host.files().read_text_many(paths);
        let checked: Vec<Result<CheckedTrip, Failure>> = paths
            .iter()
            .zip(texts)
            .map(|(path, text)| {
                text.map(|text| CheckedTrip::from(TripSource::new(text.as_ref()).findings()))
                    .map_err(|failure: FileFailure| Failure::Io {
                        site: Site::from(path.as_path()),
                        error: io::Error::from(failure),
                    })
            })
            .collect();
        let reached = checked
            .iter()
            .position(|trip| {
                trip.as_ref()
                    .ok()
                    .and_then(|trip| trip.verdict().ok())
                    .is_none()
            })
            .map_or(checked.len(), |last| last + 1);
        host.said_all(checked.iter().take(reached).flatten())?;
        checked
            .into_iter()
            .take(reached)
            .try_for_each(|trip| trip?.verdict())
    }
}

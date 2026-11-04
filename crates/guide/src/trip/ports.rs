use std::fs;
use std::path::Path;

use super::{Trip, TripError};

pub struct TripSheet(String);

impl TryFrom<TripSheet> for Trip {
    type Error = TripError;

    fn try_from(sheet: TripSheet) -> Result<Trip, TripError> {
        sheet.0.parse()
    }
}

pub struct TripFile;

impl TripFile {
    pub fn read(path: &Path) -> Result<TripSheet, TripError> {
        fs::read_to_string(path)
            .map(TripSheet)
            .map_err(|error| TripError::Unreadable {
                path: path.to_path_buf(),
                error,
            })
    }
}

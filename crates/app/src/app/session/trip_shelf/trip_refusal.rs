use std::io;

use guide::TripError;
use guide::words::Spelling;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TripRefusal {
    #[error("{file} did not open. The current trip did not change.\nThe file is not plain text.")]
    NotText { file: Spelling },
    #[error("{file} did not open. The current trip did not change.\n{error}")]
    Invalid { file: Spelling, error: TripError },
    #[error(
        "{file} did not open. The current trip did not change.\nThe phone did not save it: {error}"
    )]
    NotKept { file: Spelling, error: io::Error },
}

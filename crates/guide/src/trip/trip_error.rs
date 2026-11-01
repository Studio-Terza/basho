use std::io;
use std::path::PathBuf;

use thiserror::Error;

use crate::itinerary::Finding;

#[derive(Debug, Error)]
pub enum TripError {
    #[error("trip file {}: {error}", .path.display())]
    Unreadable { path: PathBuf, error: io::Error },
    #[error("trip file: {0}")]
    Toml(toml::de::Error),
    #[error("{}", .0.iter().map(ToString::to_string).collect::<Vec<String>>().join("\n"))]
    Invalid(Vec<Finding>),
}

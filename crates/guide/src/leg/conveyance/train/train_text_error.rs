use crate::words::Spelling;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrainTextError {
    #[error("a train line needs a name")]
    NamelessLine,
    #[error("a headsign ends in 行き or 方面 after its destination, not {0:?}")]
    Headsign(Spelling),
}

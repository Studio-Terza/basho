use crate::words::Written;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoteError {
    #[error("catching note {0:?} tells the walker a direction measured from the line")]
    FromTheLine(Written),
}

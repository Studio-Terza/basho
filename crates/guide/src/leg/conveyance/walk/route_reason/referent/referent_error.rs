use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReferentError {
    #[error("a reason refers to an entry or a place part")]
    NotAName,
}

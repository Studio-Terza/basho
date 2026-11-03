use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
#[error("text holds a control character other than a newline, carriage return or tab")]
pub struct NotPlain;

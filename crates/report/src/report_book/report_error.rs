use thiserror::Error;

#[cfg(target_os = "ios")]
use crate::ReportText;

#[derive(Debug, Error)]
pub enum ReportError {
    #[error("the phone gave no home folder")]
    NoHome,
    #[cfg(target_os = "ios")]
    #[error("the phone would not keep the reports out of its backup: {0}")]
    Backup(ReportText),
    #[error("this file is not a report")]
    NotAReport,
    #[error("the report could not be written as text: {0}")]
    Text(#[from] toml::ser::Error),
    #[error("the phone refused the file: {0}")]
    File(#[from] std::io::Error),
}

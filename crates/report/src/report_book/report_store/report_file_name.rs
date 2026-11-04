use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};

use newtype::Newtype;

use crate::ReportNumber;
use crate::report_book::report_error::ReportError;

#[derive(Newtype, newtype::Arbitrary, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReportFileName(ReportNumber);

impl ReportFileName {
    const STEM: &str = "report-";
    const EXTENSION: &str = ".toml";

    #[must_use]
    pub fn under(self, folder: &Path) -> PathBuf {
        folder.join(self.to_string())
    }
}

impl fmt::Display for ReportFileName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{:0>4}{}", Self::STEM, self.0, Self::EXTENSION)
    }
}

impl TryFrom<&OsStr> for ReportFileName {
    type Error = ReportError;

    fn try_from(name: &OsStr) -> Result<ReportFileName, ReportError> {
        name.to_str()
            .and_then(|name| name.strip_prefix(Self::STEM))
            .and_then(|rest| rest.strip_suffix(Self::EXTENSION))
            .and_then(|digits| digits.parse().ok())
            .map(|number| ReportFileName(ReportNumber::new(number)))
            .ok_or(ReportError::NotAReport)
    }
}

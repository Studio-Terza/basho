use std::fmt;

use serde::Serialize;

use crate::Report;

#[derive(newtype::Newtype, Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExportedReports {
    reports: Vec<Report>,
}

impl fmt::Display for ExportedReports {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = toml::to_string(self).map_err(|_| fmt::Error)?;
        formatter.write_str(&text)
    }
}

use std::fmt;

use newtype::proptest::arbitrary::any;
use newtype::proptest::collection::vec;
use newtype::proptest::strategy::Strategy;
use serde::Serialize;

use crate::{Capture, Report, ReportNumber, ReportText};

#[derive(newtype::Newtype, newtype::Arbitrary, Clone, Debug, PartialEq, Eq, Serialize)]
#[newtype(strategy = ExportedReports::drawn())]
pub struct ExportedReports {
    reports: Vec<Report>,
}

impl ExportedReports {
    const MOST_DRAWN: usize = 4;

    fn drawn() -> impl Strategy<Value = Vec<Report>> {
        vec(
            (
                any::<ReportNumber>(),
                any::<ReportText>(),
                any::<ReportText>(),
            )
                .prop_map(|(number, time, note)| {
                    Report::new(number, Capture::new(time)).noted(note)
                }),
            0..Self::MOST_DRAWN,
        )
    }
}

impl fmt::Display for ExportedReports {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = toml::to_string(self).map_err(|_| fmt::Error)?;
        formatter.write_str(&text)
    }
}

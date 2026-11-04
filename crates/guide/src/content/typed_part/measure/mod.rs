mod joint;
mod span_unit;

use std::fmt;
use std::str::FromStr;

use serde::Deserialize;

use crate::content::{ContentError, Figure};
use crate::words::{Spelling, Written};
use joint::Joint;

pub use span_unit::SpanUnit;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String", bound(deserialize = "Unit: FromStr"))]
pub struct Measure<Unit> {
    figure: Figure,
    joint: Joint,
    unit: Unit,
}

impl<Unit: FromStr> FromStr for Measure<Unit> {
    type Err = ContentError;

    fn from_str(text: &str) -> Result<Measure<Unit>, ContentError> {
        let bad = || ContentError::BadMeasure(Written::new(text.to_owned()));
        let figure: Figure = text.parse()?;
        let rest = text
            .get(AsRef::<Spelling>::as_ref(&figure).len()..)
            .ok_or_else(bad)?;
        let joint: Joint = rest.into();
        let unit = rest
            .strip_prefix(<&str>::from(joint))
            .and_then(|unit| unit.parse().ok())
            .ok_or_else(bad)?;
        Ok(Measure {
            figure,
            joint,
            unit,
        })
    }
}

impl<Unit: FromStr> TryFrom<String> for Measure<Unit> {
    type Error = ContentError;

    fn try_from(text: String) -> Result<Measure<Unit>, ContentError> {
        text.parse()
    }
}

impl<Unit: fmt::Display> fmt::Display for Measure<Unit> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}{}", self.figure, self.joint, self.unit)
    }
}

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

impl<Unit: FromStr> Measure<Unit> {
    fn read(text: &str) -> Option<Measure<Unit>> {
        let figure = Figure::leading(text)?;
        let rest = text.get(AsRef::<Spelling>::as_ref(&figure).len()..)?;
        let joint: Joint = rest.into();
        let unit = rest.strip_prefix(<&str>::from(joint))?.parse().ok()?;
        Some(Measure {
            figure,
            joint,
            unit,
        })
    }
}

impl<Unit: FromStr> TryFrom<String> for Measure<Unit> {
    type Error = ContentError;

    fn try_from(text: String) -> Result<Measure<Unit>, ContentError> {
        Self::read(&text).ok_or(ContentError::BadMeasure(Written::new(text)))
    }
}

impl<Unit: fmt::Display> fmt::Display for Measure<Unit> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}{}", self.figure, self.joint, self.unit)
    }
}

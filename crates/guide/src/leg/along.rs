use serde::Deserialize;

use crate::Source;
use crate::clock::Minutes;
use crate::content::Content;
use crate::geo::{Coordinate, Distance};
use crate::leg::cutoff::Cutoff;
use crate::leg::stop_kind::StopKind;
use crate::place::Fact;
use crate::ruby_text::RubyText;
use crate::times::ListedTime;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Along {
    #[serde(rename = "km", deserialize_with = "Distance::deserialize_kilometers")]
    distance: Distance,
    kind: StopKind,
    #[as_ref]
    name: Content,
    expected: Option<ListedTime>,
    dwell: Option<Minutes>,
    detail: Option<Fact>,
    cutoff: Option<Cutoff>,
    #[as_ref]
    at: Coordinate,
    #[as_ref]
    located: Source,
}

impl AsRef<RubyText> for Along {
    fn as_ref(&self) -> &RubyText {
        self.name.shown()
    }
}

impl Along {
    #[must_use]
    pub fn distance_along(&self) -> Distance {
        self.distance
    }

    #[must_use]
    pub fn kind(&self) -> StopKind {
        self.kind
    }

    #[must_use]
    pub fn expected(&self) -> Option<ListedTime> {
        self.expected
    }

    #[must_use]
    pub fn dwell_minutes(&self) -> Minutes {
        self.dwell.unwrap_or_default()
    }

    #[must_use]
    pub fn detail(&self) -> Option<&Fact> {
        self.detail.as_ref()
    }

    #[must_use]
    pub fn cutoff(&self) -> Option<&Cutoff> {
        self.cutoff.as_ref()
    }
}

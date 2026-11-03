use serde::Deserialize;
use std::ops::Index;

use crate::content::Content;
use crate::itinerary::day::Caption;
use crate::itinerary::day::goal::Reason;
use crate::leg::LegIndex;
use crate::ruby_text::RubyText;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Aim {
    until: LegIndex,
    name: RubyText,
    #[serde(rename = "for")]
    reason: Reason,
    #[as_ref]
    why: Content,
}

impl Aim {
    #[must_use]
    pub fn until(&self) -> LegIndex {
        self.until
    }

    #[must_use]
    pub fn reason(&self) -> &Reason {
        &self.reason
    }
}

impl Index<Caption> for Aim {
    type Output = RubyText;

    fn index(&self, caption: Caption) -> &RubyText {
        match caption {
            Caption::Name => &self.name,
        }
    }
}

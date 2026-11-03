mod flight;

use serde::Deserialize;

use crate::clock::{ClockTime, Minutes};
use crate::end::End;
use crate::ruby_text::RubyText;
use crate::times::ListedTime;
use flight::Flight;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Landing {
    flight: Flight,
    lands: ListedTime,
    airport: RubyText,
}

impl Landing {
    pub const THROUGH_ARRIVALS: Minutes = Minutes::new(45);

    #[must_use]
    pub fn flight(&self) -> &Flight {
        &self.flight
    }

    #[must_use]
    pub fn airport(&self) -> &RubyText {
        &self.airport
    }

    #[must_use]
    pub fn lands(&self) -> ListedTime {
        self.lands
    }

    #[must_use]
    pub fn time(&self, end: End) -> ClockTime {
        match end {
            End::Start => ClockTime::from(self.lands),
            End::Finish => ClockTime::from(self.lands).later_by(Self::THROUGH_ARRIVALS),
        }
    }
}

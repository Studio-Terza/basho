mod url;

use crate::clock::CalendarDate;
use serde::Deserialize;

pub use url::Url;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    url: Url,
    checked: CalendarDate,
}

impl Source {
    #[must_use]
    pub fn url(&self) -> &Url {
        &self.url
    }

    #[must_use]
    pub fn checked(&self) -> CalendarDate {
        self.checked
    }
}

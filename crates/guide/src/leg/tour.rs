use serde::Deserialize;

use crate::times::ListedTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tour {
    check_in: ListedTime,
}

impl Tour {
    #[must_use]
    pub fn check_in(self) -> ListedTime {
        self.check_in
    }
}

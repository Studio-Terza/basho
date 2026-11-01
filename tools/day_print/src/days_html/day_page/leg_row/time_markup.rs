use std::fmt;

use guide::classed::Classed;
use guide::times::ListedTime;
use guide::times::ShownTime;

pub struct TimeMarkup(ShownTime);

impl TimeMarkup {
    #[must_use]
    pub fn new(time: ListedTime) -> TimeMarkup {
        TimeMarkup(ShownTime::from(time))
    }
}

impl fmt::Display for TimeMarkup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "<span class=\"{}\">{}</span>",
            self.0.class(),
            self.0
        )
    }
}

use std::fmt;

use guide::reading::Reading;
use wiki::entry::Entry;

use crate::app::screen::Screen;

pub struct ScreenText {
    screen: Screen,
    page: Option<String>,
}

impl ScreenText {
    pub fn new(screen: Screen, entry: Option<&Entry>) -> ScreenText {
        ScreenText {
            screen,
            page: entry.map(|entry| {
                entry.english().map_or_else(
                    || entry.name().text(Reading::Written).to_string(),
                    ToString::to_string,
                )
            }),
        }
    }
}

impl fmt::Display for ScreenText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.screen)?;
        self.page
            .as_ref()
            .map_or(Ok(()), |name| write!(formatter, " ({name})"))
    }
}

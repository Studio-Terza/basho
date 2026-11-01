mod day_page;
use guide::english_name::EnglishName;
use std::fmt;

use guide::itinerary::Itinerary;

use day_page::DayPage;

pub struct DaysHtml<'itinerary>(&'itinerary Itinerary);

impl<'itinerary> DaysHtml<'itinerary> {
    const STYLE: &str = include_str!("../assets/print.css");

    #[must_use]
    pub fn new(itinerary: &'itinerary Itinerary) -> DaysHtml<'itinerary> {
        DaysHtml(itinerary)
    }
}

impl fmt::Display for DaysHtml<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>{} days</title><style>{}</style></head><body>",
            AsRef::<EnglishName>::as_ref(self.0.head()),
            Self::STYLE
        )?;
        self.0
            .days()
            .iter()
            .try_for_each(|day| write!(formatter, "{}", DayPage::new(day)))?;
        formatter.write_str("</body></html>")
    }
}

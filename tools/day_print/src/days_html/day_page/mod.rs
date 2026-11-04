mod escaped;
mod fact_markup;
mod leg_number;
mod leg_row;
mod ruby_markup;
mod section_sketch;
mod unverified_markup;
use guide::content::Content;
use guide::english_name::EnglishName;
use guide::reading::Reading;
use std::fmt;

use guide::itinerary::{Caption, Day};
use guide::leg::LegIndex;

use escaped::Escaped;
use fact_markup::FactMarkup;
use leg_number::LegNumber;
use leg_row::LegRow;
use ruby_markup::RubyMarkup;
use section_sketch::SectionSketch;
use unverified_markup::UnverifiedMarkup;

pub struct DayPage<'day> {
    day: &'day Day,
}

impl<'day> DayPage<'day> {
    #[must_use]
    pub fn new(day: &'day Day) -> DayPage<'day> {
        DayPage { day }
    }

    fn sketches(&self) -> Vec<SectionSketch<'day>> {
        self.day
            .legs()
            .chunk_by(|one, other| one.mode() == other.mode())
            .scan(0, |start, run| {
                let first = LegNumber::from_index(LegIndex::new(*start));
                *start += run.len();
                Some(SectionSketch::new(first, run))
            })
            .filter_map(SectionSketch::walking)
            .collect()
    }

    fn night(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.day.night().map_or(Ok(()), |night| {
            write!(
                formatter,
                "<div class=\"night\"><b>Tonight:</b> {} ({}){}<ul>",
                RubyMarkup::new(&night[Caption::Name]),
                Escaped::new(AsRef::<EnglishName>::as_ref(night).as_ref()),
                UnverifiedMarkup::new(night.as_ref())
            )?;
            night
                .notes()
                .iter()
                .try_for_each(|note| write!(formatter, "<li>{}</li>", FactMarkup::new(note)))?;
            formatter.write_str("</ul></div>")
        })
    }
}

impl fmt::Display for DayPage<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "<section class=\"day\"><h1>Day {} · {} · <span class=\"ja\">{}</span></h1><p class=\"title\">{} · {:.0} km on foot</p><p>{}</p>",
            self.day.number(),
            self.day.date().short(),
            RubyMarkup::new(&self.day[Caption::Name]),
            Escaped::new(self.day.title().text(Reading::Written).as_ref()),
            self.day.walked().kilometers(),
            RubyMarkup::new(AsRef::<Content>::as_ref(self.day)),
        )?;
        self.night(formatter)?;
        self.sketches()
            .iter()
            .try_for_each(|sketch| write!(formatter, "{sketch}"))?;
        formatter.write_str("<table>")?;
        self.day
            .legs()
            .iter()
            .enumerate()
            .try_for_each(|(index, leg)| {
                write!(
                    formatter,
                    "{}",
                    LegRow::new(LegNumber::from_index(LegIndex::new(index)), leg)
                )
            })?;
        formatter.write_str("</table></section>")
    }
}

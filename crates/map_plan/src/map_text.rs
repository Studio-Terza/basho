use guide::Elevation;
use guide::english_name::EnglishName;
use guide::itinerary::day::goal::LoopNumber;
use guide::times::{ListedTime, PlanTime};
use guide::words::{Label, Spelling};
use serde::{Serialize, Serializer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapText {
    number: Option<LoopNumber>,
    lead: Option<ListedTime>,
    text: String,
}

impl MapText {
    const MOUNT: &str = "Mount ";

    #[must_use]
    pub fn plain(text: impl fmt::Display) -> MapText {
        MapText {
            number: None,
            lead: None,
            text: text.to_string(),
        }
    }

    #[must_use]
    pub fn marked(text: impl fmt::Display, mark: Option<Label>) -> MapText {
        MapText::plain(mark.map_or_else(|| text.to_string(), |mark| format!("{text} {mark}")))
    }

    #[must_use]
    pub fn lead(&self) -> Option<ListedTime> {
        self.lead
    }

    #[must_use]
    pub fn rest(&self) -> Spelling {
        Spelling::new(self.text.clone())
    }

    #[must_use]
    pub fn landmark(
        tick: impl fmt::Display,
        time: Option<ListedTime>,
        name: impl fmt::Display,
    ) -> MapText {
        MapText {
            number: None,
            lead: time,
            text: format!("{tick}{name}"),
        }
    }

    #[must_use]
    pub fn loop_stop(number: LoopNumber, time: PlanTime, name: impl fmt::Display) -> MapText {
        MapText {
            number: Some(number),
            lead: Some(ListedTime::from(time)),
            text: name.to_string(),
        }
    }

    #[must_use]
    pub fn peak(english: &EnglishName, elevation: Elevation) -> MapText {
        MapText::plain(format!(
            "{} {}",
            Self::without_mount(english.as_ref()),
            elevation.meters()
        ))
    }

    fn without_mount(english: &str) -> &str {
        english.strip_prefix(Self::MOUNT).unwrap_or(english)
    }
}

impl Serialize for MapText {
    fn serialize<Target: Serializer>(&self, target: Target) -> Result<Target::Ok, Target::Error> {
        target.collect_str(self)
    }
}

impl fmt::Display for MapText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.number
            .map_or(Ok(()), |number| write!(formatter, "{number} · "))?;
        self.lead
            .map_or(Ok(()), |lead| write!(formatter, "{lead} "))?;
        formatter.write_str(&self.text)
    }
}

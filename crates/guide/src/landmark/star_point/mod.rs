mod point_source;
use crate::content::{Content, ContentList};
use crate::words::StarName;
use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::Source;
use crate::day_number::DayNumber;
use crate::english_name::EnglishName;
use crate::geo::Coordinate;
use crate::naming::Naming;
use crate::ruby_text::RubyText;
use point_source::PointSource;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StarPoint {
    #[expect(dead_code, reason = "kept so the file loads")]
    precision: Option<IgnoredAny>,
    #[expect(dead_code, reason = "kept so the file loads")]
    route_km: Option<IgnoredAny>,
    name: RubyText,
    english: EnglishName,
    short: StarName,
    #[as_ref]
    day: DayNumber,
    what: Content,
    #[as_ref]
    at: Coordinate,
    located: Source,
    source: Source,
    unverified: Option<Content>,
}

impl StarPoint {
    #[must_use]
    pub fn gloss(&self) -> &Content {
        &self.what
    }

    #[must_use]
    pub fn text(&self, part: Naming) -> &RubyText {
        match part {
            Naming::Name => &self.name,
        }
    }

    #[must_use]
    pub fn english(&self) -> &EnglishName {
        &self.english
    }

    #[must_use]
    pub fn short(&self) -> &StarName {
        &self.short
    }

    #[must_use]
    pub fn short_english(&self) -> EnglishName {
        EnglishName::new(self.short.as_ref().to_owned())
    }

    #[must_use]
    pub fn source(&self, backing: PointSource) -> &Source {
        match backing {
            PointSource::Location => &self.located,
            PointSource::Account => &self.source,
        }
    }

    #[must_use]
    pub fn unverified(&self) -> Option<&Content> {
        self.unverified.as_ref()
    }
}

impl<'star> From<&'star StarPoint> for ContentList<'star> {
    fn from(star: &'star StarPoint) -> ContentList<'star> {
        std::iter::once(star.gloss())
            .chain(star.unverified())
            .collect()
    }
}

use newtype::Newtype;
use std::fmt;

#[derive(Newtype)]
pub struct Uncited<'text> {
    markup: &'text str,
}

impl Uncited<'_> {
    const SOURCES: [&'static str; 3] = ["OpenStreetMap", "OSM", "GSI"];
    const MAP: &'static str = "the map";
    const MAP_OPENING: &'static str = "The map";
    const SENTENCE_END: &'static str = ". ";

    fn untagged(&self) -> String {
        self.markup.split_once('(').map_or_else(
            || self.markup.to_owned(),
            |(lead, rest)| {
                rest.split('(')
                    .fold(lead.to_owned(), Self::joined_unless_cited)
            },
        )
    }

    fn joined_unless_cited(text: String, piece: &str) -> String {
        let cited = piece.split_once(')').filter(|(inside, _)| {
            Self::SOURCES.contains(&inside.split_whitespace().next().unwrap_or_default())
        });
        match cited {
            Some((_, after)) => text.trim_end().to_owned() + after,
            None => text + "(" + piece,
        }
    }

    fn renamed(text: &str, source: &str) -> String {
        text.split_once(source).map_or_else(
            || text.to_owned(),
            |(lead, rest)| {
                rest.split(source).fold(lead.to_owned(), |before, piece| {
                    let name = Self::name_after(&before);
                    before + name + piece
                })
            },
        )
    }

    fn name_after(before: &str) -> &'static str {
        if before.trim_end().is_empty() || before.ends_with(Self::SENTENCE_END) {
            Self::MAP_OPENING
        } else {
            Self::MAP
        }
    }
}

impl fmt::Display for Uncited<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let named = Self::SOURCES
            .iter()
            .fold(self.untagged(), |text, source| Self::renamed(&text, source));
        formatter.write_str(&named)
    }
}

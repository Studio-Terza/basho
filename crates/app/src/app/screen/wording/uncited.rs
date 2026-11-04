use super::markup::Markup;
use guide::words::Label;
use newtype::Newtype;
use std::fmt;

#[derive(Newtype)]
pub struct Uncited<'text> {
    markup: &'text Markup,
}

impl Uncited<'_> {
    const SOURCES: [Label; 3] = [
        Label::new("OpenStreetMap"),
        Label::new("OSM"),
        Label::new("GSI"),
    ];
    const MAP: Label = Label::new("the map");
    const MAP_OPENING: Label = Label::new("The map");
    const SENTENCE_END: &'static str = ". ";

    fn untagged(&self) -> Markup {
        self.markup.split_once('(').map_or_else(
            || Markup::new(String::from(&**self.markup)),
            |(lead, rest)| {
                rest.split('(')
                    .map(|piece| Markup::new(piece.to_owned()))
                    .fold(Markup::new(lead.to_owned()), |text, piece| {
                        Self::joined_unless_cited(&text, &piece)
                    })
            },
        )
    }

    fn joined_unless_cited(text: &Markup, piece: &Markup) -> Markup {
        piece
            .split_once(')')
            .filter(|(inside, _)| {
                let first = inside.split_whitespace().next().unwrap_or_default();
                Self::SOURCES.iter().any(|source| source.as_ref() == first)
            })
            .map_or_else(
                || Markup::new(format!("{text}({piece}")),
                |(_, after)| Markup::new(format!("{}{after}", text.trim_end())),
            )
    }

    fn renamed(text: &Markup, source: &Label) -> Markup {
        text.split_once(source.as_ref()).map_or_else(
            || Markup::new(String::from(&**text)),
            |(lead, rest)| {
                rest.split(source.as_ref())
                    .fold(Markup::new(lead.to_owned()), |before, piece| {
                        let name = Self::name_after(&before);
                        Markup::new(format!("{before}{name}{piece}"))
                    })
            },
        )
    }

    fn name_after(before: &Markup) -> Label {
        Some(before)
            .filter(|text| text.trim_end().is_empty())
            .or(Some(before).filter(|text| text.ends_with(Self::SENTENCE_END)))
            .map_or(Self::MAP, |_| Self::MAP_OPENING)
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

use std::fmt::{self, Write};

const ENTITIES: [(char, &str); 4] = [
    ('&', "&amp;"),
    ('<', "&lt;"),
    ('>', "&gt;"),
    ('"', "&quot;"),
];

pub(super) enum Piece {
    Entity(&'static str),
    Plain(char),
}

impl From<char> for Piece {
    fn from(character: char) -> Piece {
        ENTITIES
            .iter()
            .find(|(special, _)| *special == character)
            .map_or(Piece::Plain(character), |(_, entity)| Piece::Entity(entity))
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Entity(entity) => formatter.write_str(entity),
            Piece::Plain(character) => formatter.write_char(*character),
        }
    }
}

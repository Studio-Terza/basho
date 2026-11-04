use newtype::Newtype;
use std::fmt;

#[derive(Newtype)]
pub struct Escaped<'text>(&'text str);

impl Escaped<'_> {
    const ENTITIES: [(char, &'static str); 4] = [
        ('&', "&amp;"),
        ('<', "&lt;"),
        ('>', "&gt;"),
        ('"', "&quot;"),
    ];
}

impl fmt::Display for Escaped<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.chars().try_for_each(|character| {
            let mut buffer = [0; char::MAX_LEN_UTF8];
            let raw: &str = character.encode_utf8(&mut buffer);
            formatter.write_str(
                Self::ENTITIES
                    .iter()
                    .find(|(found, _)| *found == character)
                    .map_or(raw, |(_, entity)| entity),
            )
        })
    }
}

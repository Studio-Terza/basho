use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Script {
    Kanji,
    Kana,
    Other,
}

impl Script {
    const RANGES: [(RangeInclusive<char>, Script); 6] = [
        ('\u{3400}'..='\u{4DBF}', Script::Kanji),
        ('\u{4E00}'..='\u{9FFF}', Script::Kanji),
        ('\u{F900}'..='\u{FAFF}', Script::Kanji),
        ('々'..='々', Script::Kanji),
        ('\u{3041}'..='\u{309F}', Script::Kana),
        ('\u{30A0}'..='\u{30FF}', Script::Kana),
    ];
}

impl From<char> for Script {
    fn from(character: char) -> Script {
        Self::RANGES
            .iter()
            .find(|(range, _)| range.contains(&character))
            .map_or(Script::Other, |(_, script)| *script)
    }
}

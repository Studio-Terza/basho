#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Script {
    Kanji,
    Kana,
    Other,
}

impl From<char> for Script {
    fn from(character: char) -> Script {
        match character {
            '\u{3400}'..='\u{4DBF}' | '\u{4E00}'..='\u{9FFF}' | '\u{F900}'..='\u{FAFF}' | '々' => {
                Script::Kanji
            }
            '\u{3041}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}' => Script::Kana,
            _ => Script::Other,
        }
    }
}

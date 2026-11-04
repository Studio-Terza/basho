use super::NotPlain;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Letter {
    Text,
    Layout,
    Control,
}

impl Letter {
    const LAYOUT: [char; 3] = ['\n', '\r', '\t'];

    pub fn plain(self) -> Result<(), NotPlain> {
        match self {
            Letter::Text | Letter::Layout => Ok(()),
            Letter::Control => Err(NotPlain),
        }
    }
}

impl From<char> for Letter {
    fn from(letter: char) -> Letter {
        let layout = Self::LAYOUT.contains(&letter).then_some(Letter::Layout);
        let control = letter.is_ascii_control().then_some(Letter::Control);
        layout.or(control).unwrap_or(Letter::Text)
    }
}

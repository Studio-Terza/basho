use std::fmt;

use guide::readout::recovery::catch_words::FeatureName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    text: String,
    reading: Option<String>,
}

impl Piece {
    fn plain(text: &str) -> Piece {
        Piece {
            text: text.to_owned(),
            reading: None,
        }
    }

    pub(super) fn parsed(text: impl AsRef<str>) -> Vec<Piece> {
        let text = text.as_ref();
        text.split('{')
            .take(1)
            .map(Piece::plain)
            .chain(text.split('{').skip(1).flat_map(|part| {
                Self::after_open(part).unwrap_or_else(|| vec![Piece::plain(&format!("{{{part}"))])
            }))
            .filter(|piece| piece.reading.is_some() || !piece.text.is_empty())
            .collect()
    }

    fn after_open(part: &str) -> Option<Vec<Piece>> {
        part.split_once('}').and_then(|(inside, rest)| {
            inside.split_once('|').map(|(written, kana)| {
                vec![
                    Piece {
                        text: written.to_owned(),
                        reading: Some(kana.to_owned()),
                    },
                    Piece::plain(rest),
                ]
            })
        })
    }

    pub fn with_reading(self, name: &FeatureName, kana: &FeatureName) -> Vec<Piece> {
        match &self.reading {
            Some(_) => vec![self],
            None => self.split_around(name.as_ref(), kana.as_ref()),
        }
    }

    fn split_around(&self, name: &str, kana: &str) -> Vec<Piece> {
        self.text
            .split(name)
            .enumerate()
            .flat_map(|(index, part)| {
                [
                    Self::reading_at_index(index, name, kana),
                    Some(Piece::plain(part)).filter(|piece| !piece.text.is_empty()),
                ]
            })
            .flatten()
            .collect()
    }

    fn reading_at_index(index: usize, name: &str, kana: &str) -> Option<Piece> {
        (index > 0).then(|| Piece {
            text: name.to_owned(),
            reading: Some(kana.to_owned()),
        })
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.reading {
            Some(kana) => write!(formatter, "{{{}|{kana}}}", self.text),
            None => write!(formatter, "{}", self.text),
        }
    }
}

use crate::place::RubyText;
use crate::trip::known_names::DraftPart;

#[derive(Debug, Default)]
pub struct DraftName {
    parts: Vec<DraftPart>,
    kept_to_index: usize,
}

impl DraftName {
    fn with_words(self, note: &RubyText, end_index: usize) -> DraftName {
        DraftName {
            parts: self
                .parts
                .into_iter()
                .chain(DraftPart::split(
                    &note.slice(self.kept_to_index, end_index).markup(),
                ))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn past(self, note: &RubyText, at_index: usize, name: Option<DraftPart>) -> DraftName {
        match name.filter(|_| at_index >= self.kept_to_index) {
            Some(name) => {
                let cut = self.with_words(note, at_index);
                DraftName {
                    kept_to_index: at_index + name.letter_length(),
                    parts: cut.parts.into_iter().chain([name]).collect(),
                }
            }
            None => self,
        }
    }

    #[must_use]
    pub fn finished(self, note: &RubyText, end_index: usize) -> Vec<DraftPart> {
        self.with_words(note, end_index).parts
    }
}

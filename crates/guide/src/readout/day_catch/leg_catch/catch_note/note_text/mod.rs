mod note_error;

use crate::content::Content;
use crate::reading::Reading;
use crate::words::{Spelling, Written};
use note_error::NoteError;

#[derive(Clone, Debug)]
pub struct NoteText(Content);

impl NoteText {
    const WALKER_ORDER: &str = "Go ";
    const SENTENCE_END: &str = ". ";

    #[must_use]
    pub fn content(self) -> Content {
        self.0
    }
}

impl TryFrom<Content> for NoteText {
    type Error = NoteError;

    fn try_from(content: Content) -> Result<NoteText, NoteError> {
        Self::checked(&content.text(Reading::Written), content)
    }
}

impl NoteText {
    fn checked(written: &Spelling, content: Content) -> Result<NoteText, NoteError> {
        match written
            .split(Self::SENTENCE_END)
            .find(|sentence| sentence.starts_with(Self::WALKER_ORDER))
        {
            Some(_) => Err(NoteError::FromTheLine(Written::new(written.to_string()))),
            None => Ok(NoteText(content)),
        }
    }
}

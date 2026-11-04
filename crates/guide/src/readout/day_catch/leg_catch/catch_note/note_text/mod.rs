mod note_error;

use crate::content::Content;
use crate::reading::Reading;
use crate::words::Written;
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
        let written = content.text(Reading::Written);
        written
            .split(Self::SENTENCE_END)
            .any(|sentence| sentence.starts_with(Self::WALKER_ORDER))
            .then(|| NoteError::FromTheLine(Written::new(written.to_string())))
            .map_or(Ok(NoteText(content)), Err)
    }
}

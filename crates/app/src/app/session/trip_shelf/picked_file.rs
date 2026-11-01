use std::borrow::Cow;
use std::io::{self, ErrorKind};
use std::path::Path;

use guide::Trip;
use guide::words::Spelling;
use serde::Deserialize;

use crate::app::session::kept_file::KeptFile;
use crate::app::session::trip_shelf::trip_input::TripInput;
use crate::app::session::trip_shelf::trip_refusal::TripRefusal;
use crate::app::session::trip_shelf::trip_text::TripText;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PickedFile {
    file: Spelling,
    text: Option<TripText>,
}

impl PickedFile {
    const BUILT_IN: &str = "The built-in trip";

    #[must_use]
    pub fn new(file: Spelling, text: Option<TripText>) -> PickedFile {
        PickedFile { file, text }
    }

    #[must_use]
    pub fn built_in() -> PickedFile {
        PickedFile::new(
            Spelling::new(Self::BUILT_IN.to_owned()),
            Some(TripText::new(Cow::Borrowed(TripInput::BUILT_IN))),
        )
    }

    #[must_use]
    pub fn text(&self) -> Option<&TripText> {
        self.text.as_ref()
    }

    fn kept(chosen: &KeptFile, text: &TripText) -> io::Result<()> {
        if *text == TripInput::built_in() {
            chosen.replaced(TripInput::OWN_COPY)
        } else {
            chosen.replaced(text)
        }
    }

    pub fn keep_under(&self, home: Option<&Path>) -> Result<(), TripRefusal> {
        home.map(TripInput::chosen_under)
            .zip(self.text.as_ref())
            .map_or_else(
                || Err(ErrorKind::NotFound.into()),
                |(chosen, text)| Self::kept(&chosen, text),
            )
            .map_err(|error| TripRefusal::NotKept {
                file: self.file.clone(),
                error,
            })
    }

    pub fn trip(&self) -> Result<Trip, TripRefusal> {
        self.text
            .as_ref()
            .ok_or_else(|| TripRefusal::NotText {
                file: self.file.clone(),
            })?
            .trip()
            .map_err(|error| TripRefusal::Invalid {
                file: self.file.clone(),
                error,
            })
    }
}

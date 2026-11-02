use std::borrow::Cow;
use std::io::{self, ErrorKind};
use storage_ports::HomeFolder;

use guide::Trip;
use guide::words::Spelling;
use serde::Deserialize;

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

    fn kept(home: &HomeFolder, text: &TripText) -> io::Result<()> {
        let chosen = TripInput::chosen_under(home);
        if *text == TripInput::built_in() {
            home.replace(chosen.as_ref(), &TripInput::OWN_COPY)
        } else {
            home.replace(chosen.as_ref(), text)
        }
    }

    pub fn keep_under(&self, home: Option<&HomeFolder>) -> Result<(), TripRefusal> {
        home.zip(self.text.as_ref())
            .map_or_else(
                || Err(ErrorKind::NotFound.into()),
                |(home, text)| Self::kept(home, text),
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

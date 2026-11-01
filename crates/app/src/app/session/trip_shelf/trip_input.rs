use std::borrow::Cow;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use guide::words::Label;
use guide::{Trip, TripError};

use crate::app::session::kept_file::KeptFile;
use crate::app::session::trip_shelf::trip_text::TripText;

pub struct TripInput;

impl TripInput {
    pub const BUILT_IN: &str = include_str!("../../../../../../data/trips/sample.toml");
    pub const ON_THE_PHONE: [&str; 2] = ["Documents", "trip.toml"];
    pub const OWN_COPY: &str = "";
    const CHOSEN: Label = Label::new("chosen-trip.toml");

    #[must_use]
    pub fn built_in() -> TripText {
        TripText::new(Cow::Borrowed(Self::BUILT_IN))
    }

    #[must_use]
    pub fn chosen_under(home: &Path) -> KeptFile {
        KeptFile::under(home, Self::CHOSEN)
    }

    pub fn loaded(named: Option<OsString>, home: Option<OsString>) -> Result<Trip, TripError> {
        Self::read(named, home)?.trip()
    }

    pub fn read(named: Option<OsString>, home: Option<OsString>) -> Result<TripText, TripError> {
        let chosen = home
            .as_ref()
            .map(|home| AsRef::<Path>::as_ref(&Self::chosen_under(Path::new(home))).to_path_buf());
        named
            .map(PathBuf::from)
            .or_else(|| {
                let on_the_phone = Self::ON_THE_PHONE
                    .iter()
                    .fold(PathBuf::from(home?), |path, part| path.join(part));
                [chosen.clone()?, on_the_phone]
                    .into_iter()
                    .find(|path| path.exists())
            })
            .map_or(Ok(Cow::Borrowed(Self::BUILT_IN)), |path| {
                fs::read_to_string(&path)
                    .map(|text| Self::resolved(text, chosen.as_ref(), &path))
                    .map_err(|error| TripError::Unreadable { path, error })
            })
            .map(TripText::new)
    }

    fn resolved(text: String, chosen: Option<&PathBuf>, path: &PathBuf) -> Cow<'static, str> {
        if text == Self::OWN_COPY && chosen == Some(path) {
            Cow::Borrowed(Self::BUILT_IN)
        } else {
            Cow::Owned(text)
        }
    }
}

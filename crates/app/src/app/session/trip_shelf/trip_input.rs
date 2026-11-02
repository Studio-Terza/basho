use std::borrow::Cow;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use guide::words::Label;
use guide::{Trip, TripError};
use storage_ports::HomeFolder;

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
    pub fn chosen_under(home: &HomeFolder) -> KeptFile {
        KeptFile::under(home, Self::CHOSEN)
    }

    pub fn loaded(named: Option<OsString>, home: Option<&HomeFolder>) -> Result<Trip, TripError> {
        Self::read(named, home)?.trip()
    }

    pub fn read(named: Option<OsString>, home: Option<&HomeFolder>) -> Result<TripText, TripError> {
        let reader = home
            .cloned()
            .unwrap_or_else(|| HomeFolder::on_disk(PathBuf::new()));
        let chosen =
            home.map(|home| AsRef::<Path>::as_ref(&Self::chosen_under(home)).to_path_buf());
        named
            .map(PathBuf::from)
            .map_or_else(
                || Self::kept(&reader, home, chosen.as_ref()),
                |path| Self::opened(&reader, path, chosen.as_ref()),
            )
            .map(TripText::new)
    }

    fn kept(
        reader: &HomeFolder,
        home: Option<&HomeFolder>,
        chosen: Option<&PathBuf>,
    ) -> Result<Cow<'static, str>, TripError> {
        chosen
            .cloned()
            .into_iter()
            .chain(home.map(|home| home.joined(Self::ON_THE_PHONE)))
            .map(|path| Self::opened(reader, path, chosen))
            .find(Self::found)
            .unwrap_or(Ok(Cow::Borrowed(Self::BUILT_IN)))
    }

    fn found(read: &Result<Cow<'static, str>, TripError>) -> bool {
        !matches!(read, Err(TripError::Unreadable { error, .. }) if error.kind() == ErrorKind::NotFound)
    }

    fn opened(
        reader: &HomeFolder,
        path: PathBuf,
        chosen: Option<&PathBuf>,
    ) -> Result<Cow<'static, str>, TripError> {
        reader
            .read(&path)
            .map(|text| Self::resolved(text, chosen, &path))
            .map_err(|error| TripError::Unreadable { path, error })
    }

    fn resolved(text: String, chosen: Option<&PathBuf>, path: &PathBuf) -> Cow<'static, str> {
        if text == Self::OWN_COPY && chosen == Some(path) {
            Cow::Borrowed(Self::BUILT_IN)
        } else {
            Cow::Owned(text)
        }
    }
}

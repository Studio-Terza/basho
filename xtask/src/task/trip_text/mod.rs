use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use guide::words::Label;
use guide::{Trip, TripSource};

use crate::task::failure::Failure;
use crate::task::workspace::Workspace;
use basho_lints::Site;

#[derive(newtype::Newtype, newtype::Display, derive_more::AsRef, Clone, Debug, PartialEq, Eq)]
#[as_ref(str)]
pub struct SectionText(String);

impl SectionText {
    #[must_use]
    pub fn spliced_into<'text>(
        &'text self,
        text: TripSource<'text>,
        opening: Label,
        closing: Option<Label>,
    ) -> TripText<'text> {
        TripText {
            text,
            opening,
            closing,
            section: self,
        }
    }
}

pub struct TripText<'text> {
    text: TripSource<'text>,
    opening: Label,
    closing: Option<Label>,
    section: &'text SectionText,
}

impl TripText<'_> {
    const BUILT_IN: &str = "data/trips/sample.toml";

    pub fn saved_if_valid(&self, path: &Path) -> Result<(), Failure> {
        let written = format!("{self}");
        written.parse::<Trip>()?;
        fs::write(path, written).map_err(|error| Failure::Io {
            site: Site::from(path),
            error,
        })
    }

    #[must_use]
    pub fn path_in(workspace: &Workspace) -> PathBuf {
        workspace.root().join(Self::BUILT_IN)
    }
}

impl fmt::Display for TripText<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text: &str = self.text.as_ref();
        let (before, onward) = text
            .split_once(AsRef::<str>::as_ref(&self.opening))
            .unwrap_or((text, ""));
        let after = self
            .closing
            .and_then(|closing| onward.find(AsRef::<str>::as_ref(&closing)))
            .and_then(|start| onward.get(start..))
            .unwrap_or_default();
        write!(formatter, "{before}\n{}{after}", self.section)
    }
}

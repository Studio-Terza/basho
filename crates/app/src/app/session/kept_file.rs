use std::path::{Path, PathBuf};
use std::str::FromStr;

use guide::words::Label;
use storage_ports::HomeFolder;

#[derive(newtype::Newtype, newtype::Arbitrary, derive_more::AsRef, Clone, Debug, PartialEq, Eq)]
#[as_ref(Path)]
pub struct KeptFile(PathBuf);

impl KeptFile {
    const FOLDER: [&str; 3] = ["Library", "Application Support", "basho"];

    #[must_use]
    pub fn under(home: &HomeFolder, name: Label) -> KeptFile {
        KeptFile(home.joined(Self::FOLDER).join(name.as_ref()))
    }

    #[must_use]
    pub fn read<Text: FromStr>(&self, home: &HomeFolder) -> Option<Text> {
        home.read(&self.0).ok()?.parse().ok()
    }
}

use std::fmt::Display;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use guide::words::Label;

#[derive(newtype::Newtype, derive_more::AsRef, Clone, Debug, PartialEq, Eq)]
#[as_ref(Path)]
pub struct KeptFile(PathBuf);

impl KeptFile {
    pub const PART: &str = "part";
    const FOLDER: [&str; 3] = ["Library", "Application Support", "basho"];

    #[must_use]
    pub fn under(home: &Path, name: Label) -> KeptFile {
        KeptFile(
            Self::FOLDER
                .iter()
                .fold(home.to_path_buf(), |path, part| path.join(part))
                .join(name.as_ref()),
        )
    }

    pub fn replaced(&self, contents: impl Display) -> io::Result<()> {
        let part = self.0.with_extension(Self::PART);
        self.0.parent().map_or(Ok(()), fs::create_dir_all)?;
        let mut file = File::create(&part)?;
        write!(file, "{contents}")?;
        fs::rename(&part, &self.0)
    }

    pub fn forget(&self) {
        fs::remove_file(&self.0).unwrap_or_default();
    }
}

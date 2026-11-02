use std::ffi::OsString;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use crate::Storage;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Disk;

impl Disk {
    const PART: &str = "part";
}

impl Storage for Disk {
    fn read(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn names_in(&self, folder: &Path) -> io::Result<Vec<OsString>> {
        fs::read_dir(folder)?
            .map(|entry| entry.map(|entry| entry.file_name()))
            .collect()
    }

    fn replace(&self, path: &Path, text: &dyn Display) -> io::Result<()> {
        path.parent().map_or(Ok(()), fs::create_dir_all)?;
        let part = path.with_extension(Self::PART);
        let mut file = File::create(&part)?;
        write!(file, "{text}")?;
        file.sync_all()?;
        fs::rename(&part, path)
    }

    fn forget(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
}

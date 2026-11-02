use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fmt::{Display, Write};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use storage_ports::{HomeFolder, Storage};

#[derive(Debug, Default)]
pub struct MemoryStorage {
    files: Mutex<BTreeMap<PathBuf, String>>,
}

impl MemoryStorage {
    const ROOT: &str = "/home";

    #[must_use]
    pub fn home() -> HomeFolder {
        HomeFolder::new(PathBuf::from(Self::ROOT), MemoryStorage::default())
    }

    fn files(&self) -> MutexGuard<'_, BTreeMap<PathBuf, String>> {
        self.files.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl Storage for MemoryStorage {
    fn read(&self, path: &Path) -> io::Result<String> {
        self.files()
            .get(path)
            .cloned()
            .ok_or_else(|| ErrorKind::NotFound.into())
    }

    fn names_in(&self, folder: &Path) -> io::Result<Vec<OsString>> {
        Ok(self
            .files()
            .keys()
            .filter(|path| path.parent() == Some(folder))
            .filter_map(|path| path.file_name().map(ToOwned::to_owned))
            .collect())
    }

    fn replace(&self, path: &Path, text: &dyn Display) -> io::Result<()> {
        let mut files = self.files();
        path.ancestors()
            .skip(1)
            .find(|folder| files.contains_key(*folder))
            .map_or(Ok(()), |_| Err(io::Error::from(ErrorKind::NotADirectory)))?;
        let mut kept = String::new();
        write!(kept, "{text}").map_err(|_| io::Error::from(ErrorKind::InvalidData))?;
        files.insert(path.to_path_buf(), kept);
        Ok(())
    }

    fn forget(&self, path: &Path) -> io::Result<()> {
        self.files()
            .remove(path)
            .map(drop)
            .ok_or_else(|| ErrorKind::NotFound.into())
    }
}

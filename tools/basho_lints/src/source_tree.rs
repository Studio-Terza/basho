use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use crate::failure::Failure;
use crate::site::Site;
use crate::source_file::SourceFile;

pub struct SourceTree;

impl SourceTree {
    const EXCLUDED_DIRECTORIES: [&str; 3] = ["target", ".git", "vendor"];

    pub fn files_under(root: &Path) -> Result<Vec<SourceFile>, Failure> {
        let mut files = Self::rust_files_under(root)?;
        files.sort();
        files
            .iter()
            .map(|file| SourceFile::read(root, file))
            .collect()
    }

    fn rust_files_under(directory: &Path) -> Result<Vec<PathBuf>, Failure> {
        let io = |error| Failure::Io {
            site: Site::from(directory),
            error,
        };
        fs::read_dir(directory)
            .map_err(io)?
            .map(|entry| {
                entry
                    .map_err(io)
                    .and_then(|entry| Self::rust_files_in(&entry))
            })
            .collect::<Result<Vec<Vec<PathBuf>>, Failure>>()
            .map(|nested| nested.into_iter().flatten().collect())
    }

    fn rust_files_in(entry: &DirEntry) -> Result<Vec<PathBuf>, Failure> {
        let path = entry.path();
        if !path.is_dir() {
            Ok(path
                .extension()
                .is_some_and(|extension| extension == "rs")
                .then_some(path)
                .into_iter()
                .collect())
        } else if Self::EXCLUDED_DIRECTORIES
            .iter()
            .any(|excluded| entry.file_name() == *excluded)
        {
            Ok(Vec::new())
        } else {
            Self::rust_files_under(&path)
        }
    }
}

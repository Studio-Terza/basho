use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use tempfile::{Builder, TempDir};

use crate::bits::RawBytes;
use crate::failure::Failure;

pub struct Staging {
    root: TempDir,
}

impl Staging {
    const PARENT: &str = "target";
    const PREFIX: &str = "testflight";

    pub fn prepare(root: &Path) -> Result<Staging, Failure> {
        let parent = root.join(Self::PARENT);
        fs::create_dir_all(&parent)
            .and_then(|()| Builder::new().prefix(Self::PREFIX).tempdir_in(&parent))
            .map(|root| Staging { root })
            .map_err(Self::io_failure(&parent))
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.root.path().join(name)
    }

    pub fn write(&self, name: &str, contents: &RawBytes) -> Result<PathBuf, Failure> {
        let path = self.path(name);
        path.parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| fs::write(&path, contents))
            .map_err(Self::io_failure(&path))?;
        Ok(path)
    }

    pub fn io_failure(path: &Path) -> impl Fn(io::Error) -> Failure {
        let path = path.to_path_buf();
        move |error| Failure::Io {
            path: path.clone(),
            error,
        }
    }
}

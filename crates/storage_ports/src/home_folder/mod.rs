mod disk;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::Storage;
use disk::Disk;

#[derive(Clone, Debug, derive_more::Deref)]
pub struct HomeFolder {
    root: PathBuf,
    #[deref(forward)]
    storage: Arc<dyn Storage>,
}

impl HomeFolder {
    #[must_use]
    pub fn new(root: PathBuf, storage: impl Storage + 'static) -> HomeFolder {
        HomeFolder {
            root,
            storage: Arc::new(storage),
        }
    }

    #[must_use]
    pub fn on_disk(root: PathBuf) -> HomeFolder {
        Self::new(root, Disk)
    }

    #[must_use]
    pub fn of_user() -> Option<HomeFolder> {
        Some(Self::on_disk(PathBuf::from(std::env::var_os("HOME")?)))
    }

    #[must_use]
    pub fn joined<Part: AsRef<Path>>(&self, parts: impl IntoIterator<Item = Part>) -> PathBuf {
        parts
            .into_iter()
            .fold(self.root.clone(), |path, part| path.join(part))
    }
}

impl PartialEq for HomeFolder {
    fn eq(&self, other: &HomeFolder) -> bool {
        self.root == other.root && Arc::ptr_eq(&self.storage, &other.storage)
    }
}

impl Eq for HomeFolder {}

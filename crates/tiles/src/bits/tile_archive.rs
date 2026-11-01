use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::fs::FileExt;
use std::path::Path;

use flate2::read::GzDecoder;

use super::{Directory, Slot, TileId};

pub struct TileArchive {
    file: File,
    directory: BTreeMap<TileId, Slot>,
}

impl TileArchive {
    pub fn open(path: &Path) -> io::Result<TileArchive> {
        let file = File::open(path)?;
        let directory = Directory::read_from(&file)?;
        Ok(TileArchive { file, directory })
    }

    #[must_use]
    pub fn tiles_len(&self) -> usize {
        self.directory.len()
    }

    pub fn tile(&self, id: TileId) -> io::Result<Option<Vec<u8>>> {
        self.directory
            .get(&id)
            .map(|slot| {
                let mut packed = vec![0; slot.length()];
                self.file.read_exact_at(&mut packed, slot.offset())?;
                let mut plain = Vec::new();
                GzDecoder::new(packed.as_slice()).read_to_end(&mut plain)?;
                Ok(plain)
            })
            .transpose()
    }
}

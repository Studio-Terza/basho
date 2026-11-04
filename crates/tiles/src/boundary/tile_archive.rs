use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

use flate2::read::GzDecoder;

use super::{Directory, Slot, TileId};

pub trait ArchiveSource {
    fn read_at(&self, buffer: &mut [u8], offset: u64) -> io::Result<()>;
}

impl ArchiveSource for Vec<u8> {
    fn read_at(&self, buffer: &mut [u8], offset: u64) -> io::Result<()> {
        usize::try_from(offset)
            .ok()
            .and_then(|start| Some(start..start.checked_add(buffer.len())?))
            .and_then(|span| self.get(span))
            .map(|found| buffer.copy_from_slice(found))
            .ok_or_else(|| io::Error::from(ErrorKind::UnexpectedEof))
    }
}

pub struct TileArchive<Source = File> {
    source: Source,
    directory: BTreeMap<TileId, Slot>,
}

impl<Source: ArchiveSource> TileArchive<Source> {
    pub fn read(source: Source) -> io::Result<TileArchive<Source>> {
        let directory = Directory::read_from(&source)?;
        Ok(TileArchive { source, directory })
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
                self.source.read_at(&mut packed, slot.offset())?;
                let mut plain = Vec::new();
                GzDecoder::new(packed.as_slice()).read_to_end(&mut plain)?;
                Ok(plain)
            })
            .transpose()
    }
}

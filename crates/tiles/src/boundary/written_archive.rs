use std::collections::BTreeMap;
use std::io::{self, Write};

use flate2::Compression;
use flate2::write::GzEncoder;

use super::{Directory, TileId};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WrittenArchive {
    packed: BTreeMap<TileId, Vec<u8>>,
}

impl WrittenArchive {
    pub fn add(&mut self, id: TileId, plain: &[u8]) -> io::Result<()> {
        let mut packing = GzEncoder::new(Vec::new(), Compression::best());
        packing.write_all(plain)?;
        self.packed.insert(id, packing.finish()?);
        Ok(())
    }

    #[must_use]
    pub fn tiles_len(&self) -> usize {
        self.packed.len()
    }

    #[must_use]
    pub fn bytes(&self) -> Vec<u8> {
        let head = Directory::HEAD_LENGTH + self.packed.len() * Directory::ENTRY_LENGTH;
        let count = u32::try_from(self.packed.len()).unwrap_or(u32::MAX);
        let table: Vec<u8> = self
            .packed
            .iter()
            .scan(head, |offset, (id, blob)| {
                let start = *offset;
                *offset += blob.len();
                Some(Directory::entry_bytes(*id, start, blob.len()))
            })
            .flatten()
            .collect();
        Directory::MAGIC
            .into_iter()
            .chain(count.to_le_bytes())
            .chain(table)
            .chain(self.packed.values().flatten().copied())
            .collect()
    }
}

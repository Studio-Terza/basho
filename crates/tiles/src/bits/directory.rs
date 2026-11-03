use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, ErrorKind};
use std::os::unix::fs::FileExt;

use crate::bits::GridAxis;
use map_class::Zoom;

use super::TileId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    offset: u64,
    length: u32,
}

impl Slot {
    #[must_use]
    pub const fn offset(self) -> u64 {
        self.offset
    }

    #[must_use]
    pub fn length(self) -> usize {
        usize::try_from(self.length).unwrap_or(usize::MAX)
    }
}

pub struct Directory {
    id: TileId,
    slot: Slot,
}

impl Directory {
    pub const MAGIC: [u8; 8] = *b"BASHOTL1";
    pub const HEAD_LENGTH: usize = Self::MAGIC.len() + 4;
    pub const ENTRY_LENGTH: usize = 21;

    pub fn read_from(file: &File) -> io::Result<BTreeMap<TileId, Slot>> {
        let mut head = [0; Self::HEAD_LENGTH];
        file.read_exact_at(&mut head, 0)?;
        let (magic, count) = head.split_at(Self::MAGIC.len());
        (magic == Self::MAGIC)
            .then_some(())
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "not a tile archive"))?;
        let count = usize::try_from(u32::from_le_bytes(Self::fixed(count)?))
            .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?;
        let mut table = vec![0; count * Self::ENTRY_LENGTH];
        file.read_exact_at(&mut table, u64::try_from(head.len()).unwrap_or(u64::MAX))?;
        table
            .as_chunks::<{ Self::ENTRY_LENGTH }>()
            .0
            .iter()
            .map(|entry| Self::entry(entry).map(|found| (found.id, found.slot)))
            .collect()
    }

    #[must_use]
    pub fn entry_bytes(id: TileId, start: usize, length: usize) -> Vec<u8> {
        [id.zoom().level()]
            .into_iter()
            .chain(id.index_along(GridAxis::Across).to_le_bytes())
            .chain(id.index_along(GridAxis::Down).to_le_bytes())
            .chain(u64::try_from(start).unwrap_or(u64::MAX).to_le_bytes())
            .chain(u32::try_from(length).unwrap_or(u32::MAX).to_le_bytes())
            .collect()
    }

    fn entry(bytes: &[u8]) -> io::Result<Directory> {
        let (zoom, rest) = bytes.split_at(1);
        let (column, rest) = rest.split_at(4);
        let (row, rest) = rest.split_at(4);
        let (offset, length) = rest.split_at(8);
        Ok(Directory {
            id: TileId::new(
                Zoom::of_level(zoom.first().copied().unwrap_or_default()),
                u32::from_le_bytes(Self::fixed(column)?),
                u32::from_le_bytes(Self::fixed(row)?),
            ),
            slot: Slot {
                offset: u64::from_le_bytes(Self::fixed(offset)?),
                length: u32::from_le_bytes(Self::fixed(length)?),
            },
        })
    }

    fn fixed<const LENGTH: usize>(bytes: &[u8]) -> io::Result<[u8; LENGTH]> {
        bytes
            .try_into()
            .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))
    }
}

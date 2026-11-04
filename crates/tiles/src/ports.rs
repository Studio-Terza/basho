use std::fs::File;
use std::io;
use std::os::unix::fs::FileExt;
use std::path::Path;

use crate::boundary::{ArchiveSource, TileArchive};

impl ArchiveSource for File {
    fn read_at(&self, buffer: &mut [u8], offset: u64) -> io::Result<()> {
        self.read_exact_at(buffer, offset)
    }
}

impl TileArchive<File> {
    pub fn open(path: &Path) -> io::Result<TileArchive<File>> {
        File::open(path).and_then(TileArchive::read)
    }
}

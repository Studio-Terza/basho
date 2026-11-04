use std::fmt::Display;
use std::io::Write;

use file_ports::Files;

use crate::task::failure::Failure;

pub struct Host<'host, Disk: Files, Console: Write> {
    files: &'host Disk,
    console: &'host mut Console,
}

impl<'host, Disk: Files, Console: Write> Host<'host, Disk, Console> {
    pub fn new(files: &'host Disk, console: &'host mut Console) -> Host<'host, Disk, Console> {
        Host { files, console }
    }

    pub fn files(&self) -> &Disk {
        self.files
    }

    pub fn wrote(&mut self, text: impl Display) -> Result<(), Failure> {
        write!(self.console, "{text}").map_err(Failure::Console)
    }

    pub fn said_all(
        &mut self,
        lines: impl IntoIterator<Item = impl Display>,
    ) -> Result<(), Failure> {
        lines
            .into_iter()
            .try_for_each(|line| writeln!(self.console, "{line}"))
            .map_err(Failure::Console)
    }
}

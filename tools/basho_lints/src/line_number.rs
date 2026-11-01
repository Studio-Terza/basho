use proc_macro2::LineColumn;

use crate::site::Site;
use crate::source_file::RelativePath;

#[derive(newtype::Display, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineNumber(usize);

impl LineNumber {
    pub const FIRST: LineNumber = LineNumber(1);

    #[must_use]
    pub fn from_index(index: usize) -> LineNumber {
        LineNumber(index + 1)
    }

    #[must_use]
    pub fn past_lines_in(self, text: impl AsRef<[u8]>) -> LineNumber {
        LineNumber(self.0 + text.as_ref().split(|byte| *byte == b'\n').count() - 1)
    }

    #[must_use]
    pub fn site(self, path: &RelativePath) -> Site {
        Site::Line(path.clone(), self)
    }
}

impl From<LineColumn> for LineNumber {
    fn from(start: LineColumn) -> LineNumber {
        LineNumber(start.line)
    }
}

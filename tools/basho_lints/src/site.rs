use std::fmt;
use std::path::Path;

use crate::line_number::LineNumber;
use crate::source_file::RelativePath;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Site {
    Workspace,
    File(RelativePath),
    Line(RelativePath, LineNumber),
}

impl From<&Path> for Site {
    fn from(path: &Path) -> Site {
        Site::File(RelativePath::new(path.display().to_string()))
    }
}

impl fmt::Display for Site {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Site::Workspace => formatter.write_str("workspace"),
            Site::File(path) => write!(formatter, "{path}"),
            Site::Line(path, line) => write!(formatter, "{path}:{line}"),
        }
    }
}

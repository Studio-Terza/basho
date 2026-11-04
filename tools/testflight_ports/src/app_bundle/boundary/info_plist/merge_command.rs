use std::fmt;
use std::path::Path;

pub struct MergeCommand<'plist>(&'plist Path);

impl<'plist> MergeCommand<'plist> {
    pub fn merging(plist: &'plist Path) -> MergeCommand<'plist> {
        MergeCommand(plist)
    }
}

impl fmt::Display for MergeCommand<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Merge {}", self.0.display())
    }
}

use std::ffi::OsString;
use std::fmt::{Debug, Display};
use std::io;
use std::path::Path;

pub trait Storage: Debug + Send + Sync {
    fn read(&self, path: &Path) -> io::Result<String>;
    fn names_in(&self, folder: &Path) -> io::Result<Vec<OsString>>;
    fn replace(&self, path: &Path, text: &dyn Display) -> io::Result<()>;
    fn forget(&self, path: &Path) -> io::Result<()>;
}

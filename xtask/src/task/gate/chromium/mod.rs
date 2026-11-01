mod revision;

use std::fs;
use std::path::{Path, PathBuf};

use revision::Revision;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chromium(PathBuf);

impl Chromium {
    const EXECUTABLE: &str =
        "chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing";

    #[must_use]
    pub fn newest(installs: impl IntoIterator<Item = PathBuf>) -> Option<Chromium> {
        installs
            .into_iter()
            .filter_map(|install| {
                install
                    .file_name()
                    .and_then(|name| name.to_str())
                    .and_then(|name| name.parse::<Revision>().ok())
                    .map(|revision| (revision, install))
            })
            .max()
            .map(|(_, install)| Chromium(install.join(Self::EXECUTABLE)))
    }

    #[must_use]
    pub fn installed_in(cache: &Path) -> Option<Chromium> {
        Self::newest(
            fs::read_dir(cache)
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|install| install.join(Self::EXECUTABLE).is_file()),
        )
    }

    #[must_use]
    pub fn binary(&self) -> &Path {
        &self.0
    }
}

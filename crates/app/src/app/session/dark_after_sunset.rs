use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use guide::words::Label;

use crate::app::session::kept_file::KeptFile;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DarkAfterSunset {
    #[default]
    Off,
    On,
}

impl DarkAfterSunset {
    pub const NAME: &str = "dark-after-sunset.txt";
    const ON: &str = "on";
    const OFF: &str = "off";

    #[must_use]
    pub fn switched(self) -> DarkAfterSunset {
        match self {
            DarkAfterSunset::Off => DarkAfterSunset::On,
            DarkAfterSunset::On => DarkAfterSunset::Off,
        }
    }

    #[must_use]
    pub fn restored_under(home: &Path) -> DarkAfterSunset {
        fs::read_to_string(AsRef::<Path>::as_ref(&KeptFile::under(
            home,
            Label::new(Self::NAME),
        )))
        .ok()
        .and_then(|text| Self::read(text.trim()))
        .unwrap_or_default()
    }

    pub fn kept_under(self, home: &Path) -> io::Result<()> {
        KeptFile::under(home, Label::new(Self::NAME)).replaced(self)
    }

    #[must_use]
    pub fn restored() -> DarkAfterSunset {
        std::env::var_os("HOME").map_or_else(DarkAfterSunset::default, |home| {
            Self::restored_under(Path::new(&home))
        })
    }

    pub fn store(self) {
        std::env::var_os("HOME")
            .map(|home| self.kept_under(Path::new(&home)).unwrap_or_default())
            .unwrap_or_default();
    }

    fn read(word: &str) -> Option<DarkAfterSunset> {
        [DarkAfterSunset::Off, DarkAfterSunset::On]
            .into_iter()
            .find(|setting| setting.to_string() == word)
    }
}

impl fmt::Display for DarkAfterSunset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            DarkAfterSunset::Off => Self::OFF,
            DarkAfterSunset::On => Self::ON,
        })
    }
}

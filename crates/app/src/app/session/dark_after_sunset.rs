use std::fmt;
use std::io::{self, ErrorKind};
use std::str::FromStr;

use guide::words::Label;
use storage_ports::HomeFolder;
use thiserror::Error;

use crate::app::session::kept_file::KeptFile;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DarkAfterSunset {
    #[default]
    Off,
    On,
}

#[derive(Debug, Error)]
#[error("the dark setting is neither on nor off")]
pub struct UnknownSetting;

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
    pub fn restored_in(home: Option<&HomeFolder>) -> DarkAfterSunset {
        home.and_then(|home| KeptFile::under(home, Label::new(Self::NAME)).read(home))
            .unwrap_or_default()
    }

    pub fn kept_in(self, home: Option<&HomeFolder>) -> io::Result<()> {
        home.ok_or_else(|| io::Error::from(ErrorKind::NotFound))
            .and_then(|home| {
                home.replace(
                    KeptFile::under(home, Label::new(Self::NAME)).as_ref(),
                    &self,
                )
            })
    }
}

impl FromStr for DarkAfterSunset {
    type Err = UnknownSetting;

    fn from_str(text: &str) -> Result<DarkAfterSunset, UnknownSetting> {
        [DarkAfterSunset::Off, DarkAfterSunset::On]
            .into_iter()
            .find(|setting| setting.to_string() == text.trim())
            .ok_or(UnknownSetting)
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

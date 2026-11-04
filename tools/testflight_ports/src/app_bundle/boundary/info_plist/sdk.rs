use std::fmt;

use super::InfoPlist;
use crate::boundary::Tool;
use crate::failure::Failure;

pub struct Sdk {
    version: String,
    build: String,
}

impl Sdk {
    const PLATFORM: &str = "iphoneos";

    pub fn query() -> Result<Sdk, Failure> {
        Ok(Sdk {
            version: Self::shown("--show-sdk-version")?,
            build: Self::shown("--show-sdk-build-version")?,
        })
    }

    pub fn record_in(&self, plist: &InfoPlist) -> Result<(), Failure> {
        plist.replace("DTPlatformVersion", &self.version)?;
        plist.replace("DTSDKName", self)?;
        plist.replace("DTSDKBuild", &self.build)?;
        plist.replace("DTPlatformBuild", &self.build)
    }

    fn shown(flag: &str) -> Result<String, Failure> {
        Tool::named("xcrun")
            .args(["--sdk", Self::PLATFORM, flag])
            .text()
            .map(|text| text.trim().to_owned())
    }
}

impl fmt::Display for Sdk {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", Self::PLATFORM, self.version)
    }
}

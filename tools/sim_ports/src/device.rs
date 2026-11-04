use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::boundary::Bounded;
use crate::failure::Failure;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Udid(String);

#[must_use]
pub struct Device {
    udid: Udid,
}

impl Device {
    const QUICK: Duration = Duration::from_mins(1);
    const BOOT: Duration = Duration::from_mins(5);
    const UDID_LENGTH: usize = 36;
    const NAME: &str = "basho-sim";
    const KIND: &str = "com.apple.CoreSimulator.SimDeviceType.iPhone-16-Plus";
    const APP: &str = "com.cinglein.basho";

    pub fn created() -> Result<Device, Failure> {
        let udid = Bounded::simctl(Self::QUICK)
            .args(["create", Self::NAME, Self::KIND])
            .printed()?;
        udid.lines()
            .map(str::trim)
            .find(|line| line.len() == Self::UDID_LENGTH)
            .map(|udid| Device {
                udid: Udid(udid.to_owned()),
            })
            .ok_or_else(|| Failure::Simulator(udid.clone()))
    }

    pub fn booted(self) -> Result<Device, Failure> {
        Bounded::simctl(Self::BOOT)
            .args(["boot", &self.udid.0])
            .printed()?;
        Bounded::simctl(Self::BOOT)
            .args(["bootstatus", &self.udid.0, "-b"])
            .printed()?;
        Ok(self)
    }

    pub fn with_app(self, app: &Path) -> Result<Device, Failure> {
        Bounded::simctl(Self::QUICK)
            .args(["install", &self.udid.0])
            .arg(app)
            .printed()?;
        Bounded::simctl(Self::QUICK)
            .args(["privacy", &self.udid.0, "grant", "location", Self::APP])
            .printed()?;
        Ok(self)
    }

    pub fn app_data(&self) -> Result<PathBuf, Failure> {
        Ok(PathBuf::from(
            Bounded::simctl(Self::QUICK)
                .args(["get_app_container", &self.udid.0, Self::APP, "data"])
                .printed()?
                .lines()
                .rfind(|line| line.starts_with('/'))
                .ok_or_else(|| Failure::Simulator("the app has no data folder".to_owned()))?,
        ))
    }
}

impl From<&Device> for Udid {
    fn from(device: &Device) -> Udid {
        device.udid.clone()
    }
}

impl Drop for Device {
    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    fn drop(&mut self) {
        ["shutdown", "delete"].into_iter().for_each(|action| {
            Bounded::simctl(Self::QUICK)
                .args([action, self.udid.0.as_str()])
                .printed()
                .map(drop)
                .unwrap_or_default();
        });
    }
}

impl fmt::Display for Udid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

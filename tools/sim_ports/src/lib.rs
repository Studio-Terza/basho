mod bounded;
mod device;
mod failure;
mod phase_length;
mod sim_project;
mod suite_run;

use std::path::Path;
use std::time::{Duration, SystemTime};

use bounded::Bounded;
use device::{Device, Udid};
use sim_project::SimProject;
use sim_suite::{CrashReports, SimFindings};
use suite_run::SuiteRun;

pub use failure::Failure;

pub struct Sim;

impl Sim {
    const CRATE: &str = "crates/app";
    const APP: &str = "target/dx/basho/release/ios/Basho.app";
    const DEPLOYMENT_TARGET: &str = "17.0";
    const BUILD: Duration = Duration::from_mins(20);
    const TESTS: Duration = Duration::from_mins(30);

    #[must_use]
    pub fn proved(root: &Path, folder: &Path) -> SimFindings {
        let started = SystemTime::now();
        let mut run = SuiteRun::default();
        let result = Self::driven(root, folder, &mut run);
        run.closed(CrashReports::written_since(started), &result)
    }

    fn driven(root: &Path, folder: &Path, run: &mut SuiteRun) -> Result<(), Failure> {
        Bounded::named("dx", Self::BUILD)
            .args(["build", "--ios", "--release"])
            .args(["--target", "aarch64-apple-ios-sim"])
            .env("IPHONEOS_DEPLOYMENT_TARGET", Self::DEPLOYMENT_TARGET)
            .within(&root.join(Self::CRATE))
            .printed()?;
        let device = Device::created()?
            .booted()?
            .with_app(&root.join(Self::APP))?;
        let data = device.app_data()?;
        let project = SimProject::unpacked(root, folder, &data)?;
        let udid = Udid::from(&device);
        run.watching(udid.clone());
        project
            .test_on(&udid, &data, Self::TESTS)
            .heard(|line| run.heard(line))
            .map(drop)
    }
}

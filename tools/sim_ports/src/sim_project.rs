use std::fmt;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::boundary::Bounded;
use crate::device::Udid;
use crate::failure::Failure;
use crate::phase_length::PhaseLength;

pub struct SimProject {
    unpacked: PathBuf,
    root: PathBuf,
    folder: PathBuf,
}

impl SimProject {
    const SOURCE: &str = "platform";
    const FIXTURES: &str = "fixtures";
    const TARGET: &str = "target/sim";
    const SAMPLE: &str = "data/trips/sample.toml";
    const PROJECT: &str = "BashoUITests.xcodeproj";

    pub fn unpacked(root: &Path, folder: &Path, data: &Path) -> Result<SimProject, Failure> {
        let unpacked = root.join(Self::TARGET).join("uitest");
        Self::copied(&root.join(Self::SOURCE), &unpacked)?;
        let documents = data.join("Documents");
        let shots = folder.join("shots");
        Self::made(&shots)?;
        Self::copied(&root.join(Self::SOURCE).join(Self::FIXTURES), &documents)?;
        let sample = root.join(Self::SAMPLE);
        [
            folder.join("sample.toml"),
            documents.join("sample-trip.toml"),
        ]
        .into_iter()
        .try_for_each(|path| {
            fs::copy(&sample, &path)
                .map(drop)
                .map_err(|error| Failure::Io { path, error })
        })?;
        Ok(SimProject {
            unpacked,
            root: root.to_path_buf(),
            folder: folder.to_path_buf(),
        })
    }

    #[must_use]
    pub fn test_on(&self, device: &Udid, data: &Path, limit: Duration) -> Bounded {
        Bounded::named("xcodebuild", limit)
            .args(["test", "-project"])
            .arg(self.unpacked.join(Self::PROJECT))
            .args(["-scheme", "BashoUITests", "-parallel-testing-enabled", "NO"])
            .arg("-destination")
            .arg(format!("platform=iOS Simulator,id={device}"))
            .arg("-derivedDataPath")
            .arg(self.root.join(Self::TARGET).join("derived"))
            .logged_to(self.folder.join("xcodebuild.log"))
            .env("TEST_RUNNER_APP_DATA", data)
            .env("TEST_RUNNER_FIXTURES", &self.folder)
            .env("TEST_RUNNER_SHOTS", self.folder.join("shots"))
            .env(
                "TEST_RUNNER_PHASE_SECONDS",
                PhaseLength::SIMULATED.to_string(),
            )
    }

    fn copied(from: &Path, to: &Path) -> Result<(), Failure> {
        Self::made(to)?;
        fs::read_dir(from)
            .map_err(|error| Failure::Io {
                path: from.to_path_buf(),
                error,
            })?
            .filter_map(Result::ok)
            .try_for_each(|entry| Self::entry_copied(&entry, to))
    }

    fn entry_copied(entry: &DirEntry, to: &Path) -> Result<(), Failure> {
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            Self::copied(&entry.path(), &target)
        } else {
            fs::copy(entry.path(), &target)
                .map(drop)
                .map_err(|error| Failure::Io {
                    path: target,
                    error,
                })
        }
    }

    fn made(folder: &Path) -> Result<(), Failure> {
        fs::create_dir_all(folder).map_err(|error| Failure::Io {
            path: folder.to_path_buf(),
            error,
        })
    }
}

impl fmt::Display for SimProject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.unpacked.display())
    }
}

use std::env;
use std::fs;
use std::path::PathBuf;

use sim_suite::CrashFile;

pub struct CrashFolder(PathBuf);

impl CrashFolder {
    const FOLDER: &str = "Library/Logs/DiagnosticReports";

    pub fn home() -> CrashFolder {
        CrashFolder(
            env::var_os("HOME")
                .map(|home| PathBuf::from(home).join(Self::FOLDER))
                .unwrap_or_default(),
        )
    }

    pub fn files(&self) -> Vec<CrashFile> {
        fs::read_dir(&self.0)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let written = entry.metadata().ok()?.modified().ok()?;
                Some(CrashFile::new(entry.path(), written))
            })
            .collect()
    }
}

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct CrashReports(Vec<PathBuf>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct CrashFile {
    path: PathBuf,
    written: SystemTime,
}

impl CrashFile {
    fn ours(&self) -> Option<&PathBuf> {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| name.to_ascii_lowercase().starts_with(CrashReports::APP))
            .filter(|_| {
                self.path
                    .extension()
                    .is_some_and(|ending| ending == CrashReports::ENDING)
            })
            .map(|_| &self.path)
    }
}

impl CrashReports {
    const APP: &str = "basho";
    const ENDING: &str = "ips";
    const FOLDER: &str = "Library/Logs/DiagnosticReports";

    #[must_use]
    pub fn written_since(started: SystemTime) -> CrashReports {
        Self::written_in(
            &env::var_os("HOME")
                .map(|home| PathBuf::from(home).join(Self::FOLDER))
                .unwrap_or_default(),
            started,
        )
    }

    #[must_use]
    pub fn written_in(folder: &Path, started: SystemTime) -> CrashReports {
        CrashReports(
            fs::read_dir(folder)
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let written = entry.metadata().ok()?.modified().ok()?;
                    Some(CrashFile {
                        path: entry.path(),
                        written,
                    })
                })
                .filter(|file| file.written >= started)
                .filter_map(|file| file.ours().cloned())
                .collect(),
        )
    }

    #[must_use]
    pub fn found(&self) -> &[PathBuf] {
        &self.0
    }
}

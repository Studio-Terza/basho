use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct CrashReports(Vec<PathBuf>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrashFile {
    path: PathBuf,
    written: SystemTime,
}

impl CrashFile {
    #[must_use]
    pub fn new(path: PathBuf, written: SystemTime) -> CrashFile {
        CrashFile { path, written }
    }

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

    #[must_use]
    pub fn written_since(files: &[CrashFile], started: SystemTime) -> CrashReports {
        CrashReports(
            files
                .iter()
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

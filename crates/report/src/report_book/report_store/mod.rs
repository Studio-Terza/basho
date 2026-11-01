#[cfg(target_os = "ios")]
mod backup_exclusion;
mod exported_reports;
mod report_file_name;

use std::collections::BTreeSet;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use newtype::Newtype;

use crate::report_book::report_error::ReportError;
use crate::{Report, ReportNumber};
#[cfg(target_os = "ios")]
use backup_exclusion::BackupExclusion;
use exported_reports::ExportedReports;
use report_file_name::ReportFileName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportStore {
    folder: PathBuf,
    exports: PathBuf,
}

impl ReportStore {
    const FOLDER: [&str; 4] = ["Library", "Application Support", "basho", "reports"];
    const EXPORTS: &str = "Documents";
    pub const EXPORT_NAME: &str = "bug-reports.toml";
    const PART: &str = "part";

    #[must_use]
    pub fn located_at(folder: PathBuf, exports: PathBuf) -> ReportStore {
        ReportStore { folder, exports }
    }

    #[must_use]
    pub fn in_home() -> Option<ReportStore> {
        let home = PathBuf::from(std::env::var_os("HOME")?);
        Some(Self::located_at(
            Self::FOLDER
                .iter()
                .fold(home.clone(), |path, part| path.join(part)),
            home.join(Self::EXPORTS),
        ))
    }

    #[must_use]
    pub fn stored_numbers(&self) -> BTreeSet<ReportNumber> {
        fs::read_dir(&self.folder)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| ReportFileName::try_from(entry.file_name().as_os_str()).ok())
            .map(ReportFileName::into_inner)
            .collect()
    }

    #[must_use]
    pub fn read(&self, number: ReportNumber) -> Option<Report> {
        fs::read_to_string(self.path_of(number))
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
    }

    pub fn keep(&self, report: &Report) -> Result<(), ReportError> {
        fs::create_dir_all(&self.folder)?;
        Self::write_whole(&self.path_of(report.number()), &toml::to_string(report)?)?;
        #[cfg(target_os = "ios")]
        BackupExclusion::applied_to(&self.folder)?;
        Ok(())
    }

    pub fn export(&self, reports: Vec<Report>) -> Result<(), ReportError> {
        fs::create_dir_all(&self.exports)?;
        Self::write_whole(
            &self.exports.join(Self::EXPORT_NAME),
            &ExportedReports::new(reports),
        )
    }

    fn path_of(&self, number: ReportNumber) -> PathBuf {
        ReportFileName::new(number).under(&self.folder)
    }

    fn write_whole(path: &Path, shown: &impl fmt::Display) -> Result<(), ReportError> {
        let part = path.with_extension(Self::PART);
        let mut file = File::create(&part)?;
        write!(file, "{shown}")?;
        file.sync_all()?;
        fs::rename(&part, path)?;
        Ok(())
    }
}

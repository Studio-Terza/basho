#[cfg(target_os = "ios")]
mod backup_exclusion;
mod exported_reports;
mod report_file_name;

use std::collections::BTreeSet;
use std::path::PathBuf;

use newtype::Newtype;
use storage_ports::HomeFolder;

use crate::report_book::report_error::ReportError;
use crate::{Report, ReportNumber};
#[cfg(target_os = "ios")]
use backup_exclusion::BackupExclusion;
use exported_reports::ExportedReports;
use report_file_name::ReportFileName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportStore {
    folder: PathBuf,
    export_file: PathBuf,
    home: HomeFolder,
}

impl ReportStore {
    const FOLDER: [&str; 4] = ["Library", "Application Support", "basho", "reports"];
    const EXPORTS: &str = "Documents";
    pub const EXPORT_NAME: &str = "bug-reports.toml";

    #[must_use]
    pub fn in_home(home: HomeFolder) -> ReportStore {
        ReportStore {
            folder: home.joined(Self::FOLDER),
            export_file: home.joined([Self::EXPORTS, Self::EXPORT_NAME]),
            home,
        }
    }

    #[must_use]
    pub fn stored_numbers(&self) -> BTreeSet<ReportNumber> {
        self.home
            .names_in(&self.folder)
            .into_iter()
            .flatten()
            .filter_map(|name| ReportFileName::try_from(name.as_os_str()).ok())
            .map(ReportFileName::into_inner)
            .collect()
    }

    #[must_use]
    pub fn read(&self, number: ReportNumber) -> Option<Report> {
        self.home
            .read(&self.path_of(number))
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
    }

    pub fn keep(&self, report: &Report) -> Result<(), ReportError> {
        self.home
            .replace(&self.path_of(report.number()), &toml::to_string(report)?)?;
        #[cfg(target_os = "ios")]
        BackupExclusion::applied_to(&self.folder)?;
        Ok(())
    }

    pub fn export(&self, reports: Vec<Report>) -> Result<(), ReportError> {
        self.home
            .replace(&self.export_file, &ExportedReports::new(reports))?;
        Ok(())
    }

    fn path_of(&self, number: ReportNumber) -> PathBuf {
        ReportFileName::new(number).under(&self.folder)
    }
}

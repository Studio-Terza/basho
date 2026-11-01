use std::path::Path;

use objc2_foundation::{NSNumber, NSString, NSURL, NSURLIsExcludedFromBackupKey};

use crate::ReportText;
use crate::report_book::report_error::ReportError;

pub struct BackupExclusion;

impl BackupExclusion {
    pub fn applied_to(folder: &Path) -> Result<(), ReportError> {
        let path = NSString::from_str(&folder.to_string_lossy());
        let url = NSURL::fileURLWithPath_isDirectory(&path, true);
        unsafe {
            url.setResourceValue_forKey_error(
                Some(&NSNumber::new_bool(true)),
                NSURLIsExcludedFromBackupKey,
            )
        }
        .map_err(|error| ReportError::Backup(ReportText::rendered(&error.localizedDescription())))
    }
}

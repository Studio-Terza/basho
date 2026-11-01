mod backup_ride;
mod branch_line;

use serde::Deserialize;

use crate::content::Content;
use crate::leg::cue::Cue;
use backup_ride::BackupRide;

pub use branch_line::BranchLine;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Branch {
    #[serde(rename = "where")]
    spot: Content,
    trigger: Content,
    when: Option<Cue>,
    main: Content,
    backup: Content,
    backup_ride: Option<BackupRide>,
}

impl Branch {
    #[must_use]
    pub fn line_text(&self, line: BranchLine) -> &Content {
        match line {
            BranchLine::Spot => &self.spot,
            BranchLine::Trigger => &self.trigger,
            BranchLine::Main => &self.main,
            BranchLine::Backup => &self.backup,
        }
    }

    #[must_use]
    pub fn cue(&self) -> Option<Cue> {
        self.when
    }

    #[must_use]
    pub fn backup_ride(&self) -> Option<BackupRide> {
        self.backup_ride
    }
}

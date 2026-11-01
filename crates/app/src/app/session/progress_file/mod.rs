pub mod kept_progress;

use std::fs;
use std::path::Path;

use guide::itinerary::{Day, Itinerary};

use guide::guidance::LegProgress;

use crate::app::session::kept_file::KeptFile;
use guide::words::Label;
use kept_progress::KeptProgress;

pub struct ProgressFile {
    file: KeptFile,
}

impl ProgressFile {
    pub const NAME: &str = "progress.txt";
    pub const PART: &str = KeptFile::PART;

    #[must_use]
    pub fn in_home() -> Option<ProgressFile> {
        Some(Self::under(Path::new(&std::env::var_os("HOME")?)))
    }

    #[must_use]
    pub fn under(home: &Path) -> ProgressFile {
        ProgressFile {
            file: KeptFile::under(home, Label::new(Self::NAME)),
        }
    }

    pub fn forget(&self) {
        self.file.forget();
    }

    #[must_use]
    pub fn restored(&self, kept: LegProgress, today: Option<&Day>) -> LegProgress {
        today
            .and_then(|day| kept.for_day(day.number()).or_else(|| self.read_for(day)))
            .unwrap_or_default()
    }

    pub fn keep_at_home(progress: LegProgress, trip: &Itinerary) {
        Self::in_home()
            .and_then(|file| file.keep(progress, trip))
            .unwrap_or_default();
    }

    #[must_use]
    pub fn keep(&self, progress: LegProgress, trip: &Itinerary) -> Option<()> {
        trip.days()
            .iter()
            .find(|day| progress.for_day(day.number()).is_some())
            .map(|day| KeptProgress::new(progress, day))
            .and_then(|kept| self.file.replaced(kept).ok())
    }

    fn read_for(&self, day: &Day) -> Option<LegProgress> {
        fs::read_to_string(AsRef::<Path>::as_ref(&self.file))
            .ok()?
            .parse::<KeptProgress>()
            .ok()?
            .for_day(day)
    }
}

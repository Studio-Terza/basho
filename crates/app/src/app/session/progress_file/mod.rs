pub mod kept_progress;

use guide::itinerary::{Day, Itinerary};
use storage_ports::HomeFolder;

use guide::guidance::LegProgress;

use crate::app::session::kept_file::KeptFile;
use guide::words::Label;
use kept_progress::KeptProgress;

pub struct ProgressFile {
    file: KeptFile,
    home: HomeFolder,
}

impl ProgressFile {
    pub const NAME: &str = "progress.txt";

    #[must_use]
    pub fn under(home: &HomeFolder) -> ProgressFile {
        ProgressFile {
            file: KeptFile::under(home, Label::new(Self::NAME)),
            home: home.clone(),
        }
    }

    pub fn forget(&self) {
        self.home.forget(self.file.as_ref()).unwrap_or_default();
    }

    #[must_use]
    pub fn restored(&self, kept: LegProgress, today: Option<&Day>) -> LegProgress {
        today
            .and_then(|day| kept.for_day(day.number()).or_else(|| self.read_for(day)))
            .unwrap_or_default()
    }

    pub fn keep_in(home: Option<&HomeFolder>, progress: LegProgress, trip: &Itinerary) {
        home.map(Self::under)
            .and_then(|file| file.keep(progress, trip))
            .unwrap_or_default();
    }

    #[must_use]
    pub fn keep(&self, progress: LegProgress, trip: &Itinerary) -> Option<()> {
        trip.days()
            .iter()
            .find(|day| progress.for_day(day.number()).is_some())
            .map(|day| KeptProgress::new(progress, day))
            .and_then(|kept| self.home.replace(self.file.as_ref(), &kept).ok())
    }

    fn read_for(&self, day: &Day) -> Option<LegProgress> {
        self.file.read::<KeptProgress>(&self.home)?.for_day(day)
    }
}

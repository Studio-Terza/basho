use std::cell::Cell;

use guide::leg::LegIndex;

use guide::guidance::LegProgress;

use guide::clock::Minutes;

use crate::ui_check::Drawing;
use basho::app::session::progress_file::ProgressFile;
use storage_fake::MemoryStorage;

const LEG: LegIndex = LegIndex::new(3);
const LATE: Minutes = Minutes::new(12);
const RECUT: &str = "legs 0000000000000000";
const FOLDER: [&str; 3] = ["Library", "Application Support", "basho"];

#[test]
fn kept_progress_comes_back_only_while_the_day_is_cut_the_same_way() {
    let trip = Drawing::BuiltIn.itinerary();
    let day = trip.days().first().unwrap();
    let home = MemoryStorage::home();
    let file = ProgressFile::under(&home);
    let path = home.joined(FOLDER).join(ProgressFile::NAME);
    let progress = LegProgress::confirmed(day.number(), LEG, LATE);
    ProgressFile::keep_in(Some(&home), progress, &trip);
    assert_eq!(file.restored(LegProgress::default(), Some(day)), progress);
    home.replace(&path, &format!("{progress}\n{RECUT}\n"))
        .unwrap();
    assert_eq!(
        file.restored(LegProgress::default(), Some(day)),
        LegProgress::default()
    );
}

#[test]
fn progress_goes_to_disk_again_only_when_it_changed() {
    let trip = Drawing::BuiltIn.itinerary();
    let day = trip.days().first().unwrap();
    let home = MemoryStorage::home();
    let path = home.joined(FOLDER).join(ProgressFile::NAME);
    let written = Cell::new(None);
    let progress = LegProgress::confirmed(day.number(), LEG, LATE);
    ProgressFile::keep_if_changed(Some(&home), &written, progress, &trip);
    let first = home.read(&path).unwrap();
    home.replace(&path, &RECUT).unwrap();
    ProgressFile::keep_if_changed(Some(&home), &written, progress, &trip);
    assert_eq!(home.read(&path).unwrap(), RECUT);
    home.replace(&path, &first).unwrap();
    assert_eq!(
        ProgressFile::under(&home).restored(LegProgress::default(), Some(day)),
        progress
    );
}

use std::path::{Path, PathBuf};

use report::{Capture, Fact, Report, ReportBook, ReportNumber, ReportStore, ReportText};
use serde::Deserialize;
use storage_fake::MemoryStorage;
use storage_ports::HomeFolder;

mod laws_of_types;

const FIRST: ReportNumber = ReportNumber::new(1);
const SECOND: ReportNumber = ReportNumber::new(2);
const FOLDER: [&str; 4] = ["Library", "Application Support", "basho", "reports"];

#[derive(Deserialize)]
struct Exported {
    reports: Vec<Report>,
}

struct Shelf {
    home: HomeFolder,
}

impl Shelf {
    fn new() -> Shelf {
        Shelf {
            home: MemoryStorage::home(),
        }
    }

    fn folder(&self) -> PathBuf {
        self.home.joined(FOLDER)
    }

    fn exports(&self) -> PathBuf {
        self.home.joined(["Documents"])
    }

    fn book(&self) -> ReportBook {
        ReportBook::opened(Some(ReportStore::in_home(self.home.clone())))
    }

    fn capture(time: &str) -> Capture {
        Capture::new(ReportText::rendered(&time))
            .knowing(Fact::Screen, Some(ReportText::rendered(&"map")))
            .knowing(Fact::Step, Some(ReportText::rendered(&"day 2 step 3")))
            .knowing(
                Fact::Position,
                Some(ReportText::rendered(&"38.37, 141.06 ± 12 m")),
            )
            .after_decisions(vec![ReportText::rendered(&"10:15 leg 3")])
    }

    fn notice_of(book: &ReportBook) -> String {
        book.notice().map(ToString::to_string).unwrap_or_default()
    }

    fn numbers(book: &ReportBook) -> Vec<ReportNumber> {
        book.reports().iter().map(Report::number).collect()
    }

    fn files_in(&self, folder: &Path) -> Vec<String> {
        self.home
            .names_in(folder)
            .unwrap()
            .into_iter()
            .map(|name| name.into_string().unwrap())
            .collect()
    }

    fn written(&self, name: &str, text: &str) {
        self.home.replace(&self.folder().join(name), &text).unwrap();
    }
}

#[test]
fn a_captured_report_is_whole_after_the_app_opens_again() {
    let shelf = Shelf::new();
    let book = shelf
        .book()
        .captured(Shelf::capture("first"))
        .captured(Shelf::capture("second"));
    assert_eq!(book.notice(), None);
    let reopened = shelf.book();
    assert_eq!(reopened.reports(), book.reports());
    assert_eq!(Shelf::numbers(&reopened), vec![FIRST, SECOND]);
}

#[test]
fn a_note_of_several_lines_reaches_only_its_report_and_survives_reopening() {
    let shelf = Shelf::new();
    let note = ReportText::rendered(&"the \"map\" froze\nthen came back");
    let book = shelf
        .book()
        .captured(Shelf::capture("first"))
        .captured(Shelf::capture("second"))
        .noted(SECOND, &note);
    assert_eq!(book.notice(), None);
    let reopened = shelf.book();
    assert_eq!(reopened.reports()[0].note(), &ReportText::default());
    assert_eq!(reopened.reports()[1].note(), &note);
}

#[test]
fn nothing_appears_in_the_export_folder_until_the_owner_exports() {
    let shelf = Shelf::new();
    let book = shelf.book().captured(Shelf::capture("first"));
    assert_eq!(shelf.files_in(&shelf.exports()), Vec::<String>::new());
    let exported = book.exported();
    assert!(Shelf::notice_of(&exported).starts_with("Exported 1."));
    let path = shelf.exports().join(ReportStore::EXPORT_NAME);
    let text = shelf.home.read(&path).unwrap();
    let parsed: Exported = toml::from_str(&text).unwrap();
    assert_eq!(parsed.reports, exported.reports());
    assert_eq!(
        shelf.files_in(&shelf.exports()),
        vec![ReportStore::EXPORT_NAME]
    );
}

#[test]
fn a_broken_file_is_counted_kept_and_its_number_is_never_reused() {
    let shelf = Shelf::new();
    let _kept = shelf.book().captured(Shelf::capture("first"));
    shelf.written("report-0007.toml", "number = ");
    let book = shelf.book();
    assert_eq!(book.reports().len(), 1);
    assert_eq!(
        book.unreadable().map(|count| count.to_string()),
        Some("1".to_owned())
    );
    let after = book.captured(Shelf::capture("second"));
    let broken = shelf.folder().join("report-0007.toml");
    assert_eq!(shelf.home.read(&broken).unwrap(), "number = ");
    assert_eq!(after.reports().len(), 2);
    assert_eq!(
        shelf.files_in(&shelf.folder()),
        vec!["report-0001.toml", "report-0007.toml", "report-0008.toml"]
    );
}

#[test]
fn a_report_that_cannot_be_written_stays_in_the_list_and_says_so() {
    let shelf = Shelf::new();
    shelf
        .home
        .replace(&shelf.folder(), &"a file where the folder goes")
        .unwrap();
    let book = shelf.book().captured(Shelf::capture("first"));
    assert_eq!(book.reports().len(), 1);
    assert!(Shelf::notice_of(&book).starts_with("PROBLEM"));
}

#[test]
fn a_stray_file_in_the_folder_is_not_a_report() {
    let shelf = Shelf::new();
    let _kept = shelf.book().captured(Shelf::capture("first"));
    shelf.written("report-.toml", "number = 9");
    shelf.written("notes.toml", "number = 9");
    let book = shelf.book();
    assert_eq!(Shelf::numbers(&book), vec![FIRST]);
    assert_eq!(book.unreadable(), None);
}

#[test]
fn with_no_home_folder_a_capture_still_lists_and_says_so() {
    let book = ReportBook::opened(None).captured(Shelf::capture("first"));
    assert_eq!(Shelf::numbers(&book), vec![FIRST]);
    assert!(Shelf::notice_of(&book).starts_with("PROBLEM"));
}

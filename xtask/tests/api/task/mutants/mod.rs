use std::path::{Path, PathBuf};

use file_ports::{Bytes, Files, Memory};
use xtask::task::mutants::MissedReport;

const LIST: &str = "crates/guide/src/a.rs:1:1: replace f -> bool with true\n\
                    crates/guide/src/b.rs:2:2: replace g -> bool with false\n\
                    tools/day_print/src/c.rs:3:3: delete h\n\
                    xtask/src/d.rs:4:4: delete i\n\n";
const ROOT: &str = "target/mutants";
const NOTHING_MISSED: [&str; 1] = ["missed mutants: 0"];

#[test]
fn missed_lines_are_counted_under_their_crate() {
    let text = MissedReport::from(LIST).to_string();
    assert!(text.starts_with("missed mutants: 4\n"));
    assert!(text.contains("guide: 2\n"));
    assert!(text.contains("day_print: 1\n"));
    assert!(text.contains("xtask: 1\n"));
}

#[test]
fn an_empty_list_misses_nothing() {
    let text = MissedReport::from(" \n\n").to_string();
    assert_eq!(text.lines().collect::<Vec<&str>>(), NOTHING_MISSED);
}

#[test]
fn every_run_directory_adds_to_one_report() {
    let files: Memory = ["one", "two"]
        .into_iter()
        .map(|run| {
            (
                Path::new(ROOT).join(run).join("mutants.out/missed.txt"),
                Bytes::new(LIST.as_bytes().to_vec()),
            )
        })
        .collect();
    files
        .create_directories(&Path::new(ROOT).join("empty"))
        .unwrap();
    let report = MissedReport::in_directory(Path::new(ROOT), &files).unwrap();
    assert!(report.to_string().contains("guide: 4\n"));
}

#[test]
fn no_output_directory_means_no_report() {
    let report =
        MissedReport::in_directory(&PathBuf::from(ROOT).join("absent"), &Memory::default())
            .unwrap();
    assert_eq!(
        report.to_string().lines().collect::<Vec<&str>>(),
        NOTHING_MISSED
    );
}

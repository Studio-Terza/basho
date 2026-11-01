use std::fs;

use tempfile::tempdir;
use xtask::task::mutants::MissedReport;

const LIST: &str = "crates/guide/src/a.rs:1:1: replace f -> bool with true\n\
                    crates/guide/src/b.rs:2:2: replace g -> bool with false\n\
                    tools/day_print/src/c.rs:3:3: delete h\n\
                    xtask/src/d.rs:4:4: delete i\n\n";

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
    assert_eq!(
        MissedReport::from(" \n\n").to_string(),
        "missed mutants: 0\n"
    );
}

#[test]
fn every_run_directory_adds_to_one_report() {
    let root = tempdir().unwrap();
    for run in ["one", "two"] {
        let out = root.path().join(run).join("mutants.out");
        fs::create_dir_all(&out).unwrap();
        fs::write(out.join("missed.txt"), LIST).unwrap();
    }
    fs::create_dir_all(root.path().join("empty")).unwrap();
    let report = MissedReport::in_directory(root.path()).unwrap();
    assert!(report.to_string().contains("guide: 4\n"));
}

#[test]
fn no_output_directory_means_no_report() {
    let root = tempdir().unwrap();
    let report = MissedReport::in_directory(&root.path().join("absent")).unwrap();
    assert_eq!(report.to_string(), "missed mutants: 0\n");
}

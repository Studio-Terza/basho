use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

use guide::TripSource;
use xtask::task::gate::{Chromium, Commit, Detail, Outcome, Posting, Record, StatusResult, Tree};

const HASH: &str = "0123456789abcdef0123456789abcdef01234567";

struct Installs;

impl Installs {
    fn named(names: &[&str]) -> Vec<PathBuf> {
        names
            .iter()
            .map(|name| Path::new("/cache").join(name))
            .collect()
    }
}

#[test]
fn the_newest_chromium_revision_wins_by_number_not_by_text() {
    let found = Chromium::newest(Installs::named(&[
        "chromium-999",
        "chromium-1234",
        "chromium_headless_shell-2000",
        "ffmpeg-1011",
    ]))
    .unwrap();
    assert!(found.binary().starts_with("/cache/chromium-1234/"));
}

#[test]
fn no_chromium_revision_means_none() {
    assert_eq!(
        Chromium::newest(Installs::named(&["ffmpeg-1011", "chromium-x"])),
        None
    );
}

#[test]
fn a_porcelain_listing_decides_the_tree() {
    assert_eq!("\n".parse::<Tree>(), Ok(Tree::Clean));
    assert_eq!(" M src/lib.rs\n".parse::<Tree>(), Ok(Tree::Dirty));
}

#[test]
fn only_a_full_hash_is_a_commit() {
    assert!(HASH.parse::<Commit>().is_ok());
    assert!("0123abc".parse::<Commit>().is_err());
    assert!(
        "zz23456789abcdef0123456789abcdef01234567"
            .parse::<Commit>()
            .is_err()
    );
}

#[test]
fn the_record_names_the_commit_and_says_what_happened() {
    let chromium = Chromium::newest(Installs::named(&["chromium-1"])).unwrap();
    let commit = HASH.parse::<Commit>().unwrap();
    let record = Record::started(
        commit,
        Tree::Clean,
        &chromium,
        UNIX_EPOCH + Duration::from_secs(1_700_000_000),
    )
    .finished(
        Duration::from_millis(2500),
        Outcome::Failed(Detail::from(&std::fmt::Error)),
        StatusResult::WithheldFailure,
        None,
    );
    let text = record.to_string();
    assert_eq!(
        record.path_under(Path::new("/out")),
        Path::new("/out").join(HASH).join("gate.toml")
    );
    for line in [
        "procedure = \"gate\"",
        &format!("commit = \"{HASH}\""),
        "tree = \"clean\"",
        "started_unix_seconds = 1700000000",
        "elapsed_milliseconds = 2500",
        "status = \"withheld_failure\"",
        "[outcome]",
    ] {
        assert!(text.contains(line), "{line} missing from\n{text}");
    }
}

#[test]
fn the_record_of_a_passed_gate_counts_the_facts_by_evidence_kind() {
    let chromium = Chromium::newest(Installs::named(&["chromium-1"])).unwrap();
    let counts = TripSource::new("[[notes]]\nevidence = { unverified = [\"No.\"] }\n")
        .evidence_counts()
        .unwrap();
    let text = Record::started(
        HASH.parse::<Commit>().unwrap(),
        Tree::Clean,
        &chromium,
        UNIX_EPOCH,
    )
    .finished(
        Duration::ZERO,
        Outcome::Passed,
        StatusResult::Posted,
        Some(counts),
    )
    .to_string();
    for line in [
        "[evidence]",
        "unverified = 1",
        "source = 0",
        "simulator = 0",
        "owner = 0",
    ] {
        assert!(text.contains(line), "{line} missing from\n{text}");
    }
}

#[test]
fn a_status_is_posted_only_for_a_passed_gate_on_a_clean_tree() {
    let failed = Outcome::Failed(Detail::from(&std::fmt::Error));
    assert_eq!(
        Posting::decide(Tree::Clean, &Outcome::Passed),
        Posting::Post
    );
    assert_eq!(
        Posting::decide(Tree::Dirty, &Outcome::Passed),
        Posting::WithheldDirtyTree
    );
    assert_eq!(
        Posting::decide(Tree::Clean, &failed),
        Posting::WithheldFailure
    );
    assert_eq!(
        Posting::decide(Tree::Dirty, &failed),
        Posting::WithheldFailure
    );
}

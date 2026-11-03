use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

use tempfile::TempDir;

use sim_suite::TopSample;
use sim_suite::map_phase::PhaseFigures;
use sim_suite::{CrashReports, SimFindings, Standing, SuiteEar, SuiteLine, SuiteWord, TestName};

const PASSED: &str = "Test Case '-[BashoUITests.MapTests testPinch]' passed (20.1 seconds).";
const FAILED: &str = "Test Case '-[BashoUITests.WikiTests testBack]' failed (3.0 seconds).";

struct Heard;

#[derive(Default)]
struct TallyEar(SimFindings);

impl SuiteEar for TallyEar {
    fn test_finished(&mut self, test: TestName, standing: Standing) {
        self.0.heard_test(test, standing);
    }

    fn phase_started(&mut self) {}

    fn phase_ended(&mut self, _: SuiteWord) {}
}

impl Heard {
    fn findings(lines: &[&str]) -> SimFindings {
        let mut ear = TallyEar::default();
        lines
            .iter()
            .filter_map(|line| line.parse::<SuiteLine>().ok())
            .for_each(|line| line.told_to(&mut ear));
        ear.0
            .closed(CrashReports::default(), None::<&std::io::Error>)
    }

    fn written(folder: &Path, name: &str, seconds: u64) {
        let path = folder.join(name);
        let file = File::create(path).unwrap();
        file.set_modified(UNIX_EPOCH + Duration::from_secs(seconds))
            .unwrap();
    }

    fn crash_folder() -> TempDir {
        let folder = TempDir::new().unwrap();
        Self::written(folder.path(), "basho-1.ips", 1001);
        Self::written(folder.path(), "Basho-2.ips", 1000);
        Self::written(folder.path(), "basho-old.ips", 999);
        Self::written(folder.path(), "Safari-1.ips", 1001);
        Self::written(folder.path(), "basho-3.diag", 1001);
        folder
    }
}

#[test]
fn a_finished_test_line_names_its_feature_and_case() {
    let Ok(SuiteLine::Finished { test, standing }) = PASSED.parse::<SuiteLine>() else {
        panic!("a finished test");
    };
    assert!(format!("{test:?}").contains("Maps"));
    assert_eq!(test.case(), &"testPinch".parse::<SuiteWord>().unwrap());
    assert_eq!(standing, Standing::Passed);
}

#[test]
fn started_tests_and_unknown_classes_are_not_results() {
    assert_eq!(
        "Test Case '-[BashoUITests.MapTests testPinch]' started.".parse::<SuiteLine>(),
        Ok(SuiteLine::Other)
    );
    assert_eq!(
        "Test Case '-[BashoUITests.Strange testPinch]' passed (1 seconds).".parse::<SuiteLine>(),
        Ok(SuiteLine::Other)
    );
}

#[test]
fn phase_lines_name_the_phase() {
    assert_eq!(
        "PHASE START idle".parse::<SuiteLine>(),
        "idle".parse::<SuiteWord>().map(SuiteLine::PhaseStart)
    );
    assert_eq!(
        "PHASE END pan".parse::<SuiteLine>(),
        "pan".parse::<SuiteWord>().map(SuiteLine::PhaseEnd)
    );
}

#[test]
fn a_feature_with_no_tests_or_a_failure_does_not_pass() {
    let toml = toml::to_string(&Heard::findings(&[PASSED, FAILED])).unwrap_or_default();
    assert!(
        toml.contains("feature = \"maps\"\nstanding = \"passed\""),
        "{toml}"
    );
    assert!(
        toml.contains("feature = \"wiki\"\nstanding = \"failed\""),
        "{toml}"
    );
    assert!(
        toml.contains("feature = \"set_now\"\nstanding = \"failed\""),
        "{toml}"
    );
}

#[test]
fn a_full_pass_or_a_crash_decides_the_verdict() {
    let all: Vec<String> = [
        "TripPickerTests",
        "BugReportTests",
        "WikiTests",
        "MapTests",
        "SetNowTests",
        "NavigationTests",
        "MapPerfTests",
    ]
    .iter()
    .map(|class| format!("Test Case '-[BashoUITests.{class} testIt]' passed (1 seconds)."))
    .collect();
    let lines: Vec<&str> = all.iter().map(String::as_str).collect();
    assert!(Heard::findings(&lines).verdict().is_ok());
    assert!(Heard::findings(&lines[1..]).verdict().is_err());
    let folder = Heard::crash_folder();
    let crash = CrashReports::written_in(folder.path(), UNIX_EPOCH + Duration::from_secs(1000));
    assert!(
        Heard::findings(&lines)
            .closed(crash, None::<&std::io::Error>)
            .verdict()
            .is_err()
    );
}

#[test]
fn what_cannot_be_simulated_says_so() {
    let toml = toml::to_string(&Heard::findings(&[])).unwrap_or_default();
    assert!(toml.contains("battery drain"), "{toml}");
    assert!(toml.contains("standing = \"cannot_simulate\""), "{toml}");
}

#[test]
fn only_our_reports_written_since_the_start_count_as_crashes() {
    let folder = Heard::crash_folder();
    let found = CrashReports::written_in(folder.path(), UNIX_EPOCH + Duration::from_secs(1000));
    let mut names: Vec<PathBuf> = found
        .found()
        .iter()
        .filter_map(|path| path.file_name().map(PathBuf::from))
        .collect();
    names.sort();
    assert_eq!(
        names,
        [PathBuf::from("Basho-2.ips"), PathBuf::from("basho-1.ips")]
    );
}

#[test]
fn the_top_figures_are_the_last_sample_summed_over_processes() {
    let printed = "Processes: 2\nPID %CPU POWER\n11 90.0 99.0\nProcesses: 2\nPID %CPU POWER\n11 12.5 3.0\n22 7.5 1.5\n";
    let figures = printed
        .parse::<TopSample>()
        .map(|sample| PhaseFigures::from(("idle".parse().unwrap(), sample)))
        .ok();
    let shown = toml::to_string(&figures).unwrap_or_default();
    assert!(shown.contains("cpu_percent = 20.0"), "{shown}");
    assert!(shown.contains("mac_energy_impact = 4.5"), "{shown}");
}

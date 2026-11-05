use std::path::Path;

use guide::EvidenceCounts;
use guide::FactSource;
use guide::evidence::FactCount;
use guide::evidence::{Evidence, EvidenceKind};
use guide::times::FactTime;
use guide::{Trip, TripSource};

use crate::fixture::Fixture;

const SAMPLE: &str = include_str!("../../../../../data/trips/sample.toml");
const BUILT_IN: &str = include_str!("../../../../../data/trips/sample.toml");
const CITED: &str = "evidence = { source = { url = \"";
const WEB: &str = "http";
const RESEARCH: &str = "research/";
const MISSING_EVIDENCE: &str = "missing field `evidence`";
const OUTSIDE: [(&str, Cited<'static>); 2] = [(WEB, Cited::Web), (RESEARCH, Cited::Research)];

#[derive(Clone, Copy)]
enum Cited<'cited> {
    Web,
    Research,
    File(&'cited str),
}

impl<'cited> Cited<'cited> {
    fn of(cited: &'cited str) -> Self {
        OUTSIDE
            .iter()
            .find(|(prefix, _)| cited.starts_with(prefix))
            .map_or(Cited::File(cited), |(_, kind)| *kind)
    }

    fn file(self) -> Option<&'cited str> {
        match self {
            Cited::File(path) => Some(path),
            Cited::Web | Cited::Research => None,
        }
    }
}
const PLANNED: &str = "start = { plan = \"09:00\" }";
const FACT_EVIDENCE: &str = "evidence = { unverified = [\"This place is not real.\"] }";
const NIGHT_EVIDENCE: &str =
    "evidence = { unverified = [\"This place is not real.\"] }\nwhy = [\"Sleep here.\"]";
const BACKED: [(&str, EvidenceKind); 3] = [
    (
        "source = { url = \"https://example.org\", checked = \"2026-10-31\" }",
        EvidenceKind::Source,
    ),
    (
        "simulator = { url = \"research/sim_release.png\", checked = \"2026-10-31\" }",
        EvidenceKind::Simulator,
    ),
    (
        "owner = { url = \"research/notes.md\", checked = \"2026-10-31\" }",
        EvidenceKind::Owner,
    ),
];
const UNBACKED: [&str; 14] = [
    "source = {}",
    "simulator = {}",
    "owner = {}",
    "source = { checked = \"2026-10-31\" }",
    "simulator = { url = \"research/sim_release.png\" }",
    "owner = { url = \"\", checked = \"2026-10-31\" }",
    "source = { url = \"https://example.org\", checked = \"last week\" }",
    "source = \"https://example.org\"",
    "verified = true",
    "unverified = []",
    "unverified = [\"  \"]",
    "source = { url = \"https://example.org\", checked = \"2099-01-01\" }",
    "source = { url = \"planning/plan.md#day-0\", checked = \"2026-10-31\" }",
    "simulator = { url = \"planning/stay.md\", checked = \"2026-10-31\" }",
];

struct Written;

impl Written {
    fn evidence(text: &str) -> Result<Evidence, toml::de::Error> {
        toml::from_str(text)
    }

    fn unverified() -> Evidence {
        Self::evidence("unverified = [\"Nobody looked.\"]").unwrap()
    }
}

#[test]
fn every_kind_with_its_backing_loads_as_that_kind() {
    let kinds: Vec<EvidenceKind> = BACKED
        .iter()
        .map(|(text, _)| Written::evidence(text).unwrap().kind())
        .collect();
    assert_eq!(kinds, BACKED.map(|(_, kind)| kind));
}

#[test]
fn evidence_that_does_not_say_what_backs_it_fails_at_load() {
    let loaded: Vec<&str> = UNBACKED
        .into_iter()
        .filter(|text| Written::evidence(text).is_ok())
        .collect();
    assert_eq!(loaded, Vec::<&str>::new());
}

#[test]
fn a_fact_time_with_no_evidence_fails_at_load() {
    let bare = SAMPLE
        .replacen(
            PLANNED,
            "start = { fact = \"09:00\", source = \"train\" }",
            1,
        )
        .parse::<Trip>();
    assert!(bare.is_err_and(|error| error.to_string().contains("start")));
}

#[test]
fn a_night_or_a_fact_with_no_evidence_fails_at_load() {
    let spoiled = [
        SAMPLE.replacen(NIGHT_EVIDENCE, "why = [\"Sleep here.\"]", 1),
        SAMPLE.replacen(FACT_EVIDENCE, "", 1),
    ];
    let unchanged: Vec<&String> = spoiled.iter().filter(|text| *text == SAMPLE).collect();
    assert_eq!(unchanged, Vec::<&String>::new());
    let named: Vec<&String> = spoiled
        .iter()
        .filter(|text| Fixture::said(text.parse()).contains(MISSING_EVIDENCE))
        .collect();
    assert_eq!(named.len(), spoiled.len());
}

#[test]
fn a_fact_time_keeps_the_kind_of_its_evidence() {
    let noon = "12:00".to_owned().try_into().unwrap();
    assert_eq!(
        FactTime::new(noon, FactSource::Closes, &Written::unverified()).evidence(),
        Some(EvidenceKind::Unverified)
    );
    assert_eq!(FactTime::at_sunset(noon).evidence(), None);
}

#[test]
fn only_an_unverified_fact_time_reads_with_the_mark() {
    let noon = "12:00".to_owned().try_into().unwrap();
    let backed = Written::evidence(BACKED[0].0).unwrap();
    assert_eq!(
        FactTime::new(noon, FactSource::Closes, &Written::unverified()).to_string(),
        "closes 12:00 ?"
    );
    assert_eq!(
        FactTime::new(noon, FactSource::TrainTimes, &Written::unverified()).to_string(),
        "12:00 train ?"
    );
    assert_eq!(
        FactTime::new(noon, FactSource::TrainTimes, &backed).to_string(),
        "12:00 train"
    );
    assert_eq!(FactTime::at_sunset(noon).to_string(), "sunset 12:00");
}

#[test]
fn the_counts_cover_every_evidence_entry_in_the_file_by_kind() {
    let backed = BACKED
        .iter()
        .map(|(text, _)| format!("{{ text = [\"Fact.\"], evidence = {{ {text} }} }}"))
        .collect::<Vec<_>>()
        .join(", ");
    let text = format!(
        "[[days]]\nnotes = [{backed}]\n[days.night]\nevidence = {{ unverified = [\"No.\"] }}\n[places]\nfirst = {{ evidence = {{ unverified = [\"No.\"] }} }}\n"
    );
    let counts = TripSource::new(&text).evidence_counts().unwrap();
    assert_eq!(
        counts.to_string(),
        "unverified 2, source 1, simulator 1, owner 1"
    );
}

#[test]
fn the_counts_of_a_file_are_the_sum_of_its_kinds_and_stop_at_a_bad_entry() {
    let counts: EvidenceCounts = TripSource::new(BUILT_IN).evidence_counts().unwrap();
    assert_eq!(
        counts.total(),
        FactCount::new(BUILT_IN.matches("evidence = {").count())
    );
    assert!(
        TripSource::new("evidence = { source = {} }")
            .evidence_counts()
            .is_err()
    );
}

#[test]
fn a_source_in_the_repo_names_a_file_that_is_there() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let cited: Vec<&str> = BUILT_IN
        .split(CITED)
        .skip(1)
        .filter_map(|rest| rest.split(['"', '#']).next())
        .map(Cited::of)
        .filter_map(Cited::file)
        .collect();
    let there: Vec<&str> = cited
        .iter()
        .copied()
        .filter(|path| root.join(path).is_file())
        .collect();
    assert_eq!(there, cited);
}

#[test]
fn a_fact_with_no_text_fails_at_load() {
    let written = "see = { text = [\"A bridge.\"]";
    let refused: Vec<bool> = ["[\"\"]", "[\" \"]", "[]"]
        .iter()
        .map(|empty| {
            SAMPLE
                .replacen(written, &format!("see = {{ text = {empty}"), 1)
                .parse::<Trip>()
                .is_err_and(|error| error.to_string().contains("a fact needs its text"))
        })
        .collect();
    assert!(SAMPLE.contains(written));
    assert_eq!(refused, [true; 3]);
}

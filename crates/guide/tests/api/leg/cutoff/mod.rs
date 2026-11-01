use crate::fixture::{DaySource, Fixture};

const ARCS: &str = "[[arcs]]\nnumber = 1\ntitle = [\"A → B\"]\nstart = \"A\"\ndays = [3]\n";
const DAY: &str = include_str!("../../itinerary/fixture_day.toml");
const ROUTES: &str = r#"{"legs":{"1":{"segments":[{"no_mapped_path":{"via":[[140.0,38.0],[140.004,38.0]]}}]},"2":{"segments":[{"no_mapped_path":{"via":[[140.004,38.0],[140.012,38.0]]}}]}}}"#;
const PAST: &str = "after its deadline closes 8:00";
const CLOSED_EARLY: &str = "  cutoff = { kind = \"closes\", at = { fact = \"08:00\", source = \"closes\", evidence = { unverified = [\"Made up.\"] } } }\n";

#[test]
fn a_stop_that_closes_before_the_plan_reaches_it_is_reported() {
    let day = format!("{DAY}{CLOSED_EARLY}");
    let findings = Fixture::inspect(ARCS, &[DaySource::new(&day, ROUTES)]);
    assert!(
        findings
            .iter()
            .any(|finding| finding.to_string().contains(PAST)),
        "{findings:?}"
    );
}

use crate::fixture::Fixture;
use guide::itinerary::Day;
use guide::leg::Mode;
use guide::leg::conveyance::RouteReason;
use guide::{Trip, TripError};

const RECORDED: &str = r#"mode = { walk = { reason = { recorded = { kind = "quieter", refers = { entry = "Eastfield" }, evidence = { unverified = ["Stated in the leg text."] } } } } }"#;
const SPOILED: [[&str; 3]; 6] = [
    [
        "kind = \"quieter\"",
        "kind = \"scenic\"",
        "unknown variant `scenic`",
    ],
    ["kind = \"quieter\", ", "", "missing field `kind`"],
    [
        "refers = { entry = \"Eastfield\" }, ",
        "",
        "missing field `refers`",
    ],
    [
        "refers = { entry = \"Eastfield\" }",
        "refers = { plan = \"13:15\" }",
        "refers",
    ],
    [
        ", evidence = { unverified = [\"Stated in the leg text.\"] }",
        "",
        "missing field `evidence`",
    ],
    [
        "reason = { recorded",
        "reason = { stated",
        "unknown variant `stated`",
    ],
];

#[test]
fn a_bare_walk_mode_fails_at_load() {
    let bare = Fixture::with_first_walk_as("mode = \"walk\"");
    assert!(matches!(bare, Err(TripError::Toml(_))));
}

#[test]
fn a_walk_marked_none_recorded_carries_no_kind() {
    let sample = Fixture::BUNDLED.parse::<Trip>().expect("loads");
    let first = &sample.itinerary().days()[0].legs()[0];
    assert!(matches!(
        first.route_reason(),
        Some(RouteReason::NoneRecorded)
    ));
    assert!(first.route_reason().and_then(RouteReason::kind).is_none());
}

#[test]
fn a_recorded_reason_reports_its_kind_and_link() {
    let recorded = Fixture::with_first_walk_as(RECORDED).expect("loads");
    let first = recorded.itinerary().days()[0].legs()[0]
        .route_reason()
        .expect("a walk");
    assert!(first.kind().is_some());
    assert!(first.referent().is_some());
}

#[test]
fn only_walks_carry_a_reason() {
    let bundled = Fixture::bundled().expect("loads");
    assert!(
        bundled
            .days()
            .iter()
            .flat_map(Day::legs)
            .all(|leg| { (leg.mode() == Mode::Walk) == leg.route_reason().is_some() })
    );
}

#[test]
fn every_reason_in_the_bundled_trip_names_something_known() {
    let bundled = Fixture::BUNDLED.parse::<Trip>().expect("loads");
    assert_eq!(bundled.unresolved_names().len(), 0);
}

#[test]
fn a_reason_that_is_only_free_text_fails_at_load() {
    let text = Fixture::with_first_walk_as("mode = { walk = { reason = \"it is nicer\" } }");
    assert!(text.is_err_and(|error| error.to_string().contains("variant")));
}

#[test]
fn each_missing_or_unknown_part_of_a_reason_is_named_at_load() {
    let named: Vec<&str> = SPOILED
        .iter()
        .filter(|[good, bad, message]| {
            Fixture::said(Fixture::with_first_walk_as(
                &RECORDED.replacen(good, bad, 1),
            ))
            .contains(message)
        })
        .map(|[_, bad, _]| *bad)
        .collect();
    assert_eq!(named, SPOILED.map(|[_, bad, _]| bad));
}

#[test]
fn a_reason_that_refers_to_no_entry_is_found() {
    let dangling =
        Fixture::with_first_walk_as(&RECORDED.replacen("\"Eastfield\"", "\"Nowhere\"", 1))
            .expect("loads");
    assert_ne!(dangling.unresolved_names().len(), 0);
}

#[test]
fn a_recorded_reason_loads() {
    let recorded = Fixture::with_first_walk_as(RECORDED);
    assert!(recorded.is_ok(), "{:?}", recorded.err());
}

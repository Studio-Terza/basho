use guide::itinerary::Day;
use guide::leg::conveyance::Platform;

use crate::fixture::Fixture;

const PLATFORMS: usize = 5;
const CHECK_BOARD: &str = "check the board";
const WHOLE: &str = r#"mode = { train = { line = "JR {奥羽本線|おううほんせん}", kind = "local", headsign = "{新庄|しんじょう}{行|ゆ}き", platform = "not_published", evidence = { unverified = ["Made up."] } } }"#;
const SPOILED: [[&str; 3]; 13] = [
    [
        "line = \"JR {奥羽本線|おううほんせん}\", ",
        "",
        "missing field `line`",
    ],
    ["kind = \"local\", ", "", "missing field `kind`"],
    [
        "headsign = \"{新庄|しんじょう}{行|ゆ}き\", ",
        "",
        "missing field `headsign`",
    ],
    [
        ", platform = \"not_published\"",
        "",
        "missing field `platform`",
    ],
    [
        "\"JR {奥羽本線|おううほんせん}\"",
        "\"\"",
        "a train line needs a name",
    ],
    ["\"local\"", "\"fast\"", "unknown variant `fast`"],
    ["\"{新庄|しんじょう}{行|ゆ}き\"", "\"\"", "headsign"],
    ["\"{新庄|しんじょう}{行|ゆ}き\"", "\"north\"", "headsign"],
    [
        "\"{新庄|しんじょう}{行|ゆ}き\"",
        "\"{行|ゆ}き\"",
        "headsign",
    ],
    [
        "\"not_published\"",
        "{ published = { number = 3 } }",
        "missing field `evidence`",
    ],
    [
        "\"not_published\"",
        "{ published = { number = 0, evidence = { source = { url = \"https://example.org\", checked = \"2026-10-31\" } } } }",
        "nonzero",
    ],
    [
        ", evidence = { unverified = [\"Made up.\"] }",
        "",
        "missing field `evidence`",
    ],
    [
        "\"not_published\"",
        "{ published = { number = 3, evidence = { unverified = [\"A guess.\"] } } }",
        "not a guess",
    ],
];

#[test]
fn a_bare_train_mode_fails_at_load() {
    let bare = Fixture::with_first_walk_as("mode = \"train\"");
    assert!(bare.is_err_and(|error| error.to_string().contains("variant")));
}

#[test]
fn each_missing_or_empty_board_fact_is_named_at_load() {
    let named: Vec<&str> = SPOILED
        .iter()
        .filter(|[good, bad, said]| {
            Fixture::said(Fixture::with_first_walk_as(&WHOLE.replacen(good, bad, 1))).contains(said)
        })
        .map(|[_, bad, _]| *bad)
        .collect();
    assert_eq!(named, SPOILED.map(|[_, bad, _]| bad));
}

#[test]
fn a_platform_either_keeps_its_evidence_or_says_to_check_the_board() {
    let trip = Fixture::bundled().unwrap();
    let platforms: Vec<&Platform> = trip
        .days()
        .iter()
        .flat_map(Day::legs)
        .filter_map(|leg| leg.conveyance().train())
        .map(AsRef::as_ref)
        .collect();
    assert_eq!(platforms.len(), PLATFORMS);
    assert!(platforms.iter().all(
        |platform| platform.evidence().is_some() != platform.to_string().contains(CHECK_BOARD)
    ));
}

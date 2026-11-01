use guide::Trip;

const SAMPLE: &str = include_str!("../../../../../data/trips/sample.toml");
const WALK: &str = "mode = { walk = { reason = \"none_recorded\" } }";
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
    let bare = SAMPLE.replacen(WALK, "mode = \"train\"", 1).parse::<Trip>();
    assert!(bare.is_err_and(|error| error.to_string().contains("variant")));
}

#[test]
fn each_missing_or_empty_board_fact_is_named_at_load() {
    let unnamed: Vec<&str> = SPOILED
        .iter()
        .filter(|[good, bad, said]| {
            !SAMPLE
                .replacen(WALK, &WHOLE.replacen(good, bad, 1), 1)
                .parse::<Trip>()
                .map_or_else(|error| error.to_string(), |_| String::new())
                .contains(said)
        })
        .map(|[_, bad, _]| *bad)
        .collect();
    assert_eq!(unnamed, Vec::<&str>::new());
}

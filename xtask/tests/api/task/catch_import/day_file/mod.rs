mod note_text;
use std::path::Path;

use guide::Trip;
use guide::catch_words::Readings;
use guide::trip::KnownNames;
use serde_json::{Value, json};
use xtask::task::catch_import::day_file::DayFile;

const DAY: &str = r#"{"legs":[{"catch":[{"ste":"The Oakford road is 250 m north of Eastfield and Mill Inn."}]}]}"#;

#[test]
fn a_note_is_written_as_parts() {
    let trip = Trip::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/trips/sample.toml"))
        .expect("the sample loads");
    let names = KnownNames::new(
        trip.itinerary(),
        trip.places().places(),
        trip.overview().landmarks(),
    );
    let day: DayFile = DAY.parse().expect("a day");
    let written = serde_json::to_value(day.annotated(&Readings::default(), &names))
        .expect("a day writes back");
    assert_eq!(
        written
            .pointer("/legs/0/catch/0/ste")
            .cloned()
            .unwrap_or(Value::Null),
        json!([
            "The ",
            { "place": "Oakford" },
            " road is ",
            { "distance": "250 m" },
            " north of ",
            { "entry": "Eastfield" },
            " and ",
            { "entry": "Mill Inn" },
            "."
        ])
    );
}

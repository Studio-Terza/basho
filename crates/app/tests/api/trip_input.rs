use guide::english_name::EnglishName;
use std::ffi::OsString;

use basho::app::session::trip_shelf::TripInput;
use storage_fake::MemoryStorage;

const NO_SUCH_FILE: &str = "no/such/trip.toml";
const SAMPLE: &str = include_str!("../../../../data/trips/sample.toml");

#[test]
fn a_named_file_that_is_missing_fails_and_never_falls_back_to_the_built_in_trip() {
    let home = MemoryStorage::home();
    let loaded = TripInput::loaded(Some(OsString::from(NO_SUCH_FILE)), Some(&home));
    assert!(loaded.is_err());
}

#[test]
fn with_no_file_the_app_opens_the_built_in_trip() {
    let trip = TripInput::loaded(None, None).expect("the built-in trip loads");
    assert_eq!(
        AsRef::<EnglishName>::as_ref(trip.itinerary().head()).to_string(),
        "Sample"
    );
}

#[test]
fn a_trip_file_in_the_phone_documents_folder_is_opened() {
    let home = MemoryStorage::home();
    home.replace(&home.joined(TripInput::ON_THE_PHONE), &SAMPLE)
        .unwrap();
    let trip = TripInput::loaded(None, Some(&home)).unwrap();
    assert_eq!(
        AsRef::<EnglishName>::as_ref(trip.itinerary().head()).to_string(),
        "Sample"
    );
}

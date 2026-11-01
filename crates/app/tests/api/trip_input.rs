use guide::english_name::EnglishName;
use std::ffi::OsString;
use std::fs;

use basho::app::session::trip_shelf::TripInput;

const NO_SUCH_FILE: &str = "no/such/trip.toml";
const SAMPLE: &str = include_str!("../../../../data/trips/sample.toml");

#[test]
fn a_named_file_that_is_missing_fails_and_never_falls_back_to_the_built_in_trip() {
    let loaded = TripInput::loaded(Some(OsString::from(NO_SUCH_FILE)), None);
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
    let home = tempfile::tempdir().unwrap();
    let folder = home.path().join(TripInput::ON_THE_PHONE[0]);
    fs::create_dir_all(&folder).unwrap();
    fs::write(folder.join(TripInput::ON_THE_PHONE[1]), SAMPLE).unwrap();
    let trip = TripInput::loaded(None, Some(home.path().into())).unwrap();
    assert_eq!(
        AsRef::<EnglishName>::as_ref(trip.itinerary().head()).to_string(),
        "Sample"
    );
}

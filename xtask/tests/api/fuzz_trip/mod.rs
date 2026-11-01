mod fuzz_run;
mod spoil;
mod survey;

use std::fs;

use guide::Trip;
use laws::proptest::prelude::{any, prop_assert, proptest};
use spoil::Spoils;
use survey::Survey;

proptest! {
    #![proptest_config(fuzz_run::FuzzRun::config())]

    #[test]
    fn any_text_is_refused_or_safe_to_render(text in any::<String>()) {
        prop_assert!(Survey::of_text(&text).is_ok());
    }

    #[test]
    fn any_bytes_in_a_file_are_refused_or_safe_to_render(
        bytes in any::<Vec<u8>>(),
    ) {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("trip.toml");
        fs::write(&path, &bytes).unwrap();
        let loaded = Trip::load(&path);
        prop_assert!(loaded.map_or_else(|error| !error.to_string().is_empty(), |trip| Survey::of(&trip).is_ok()));
    }

    #[test]
    fn a_spoiled_sample_is_refused_or_safe_to_render(spoils in Spoils::strategy()) {
        prop_assert!(Survey::of_text(&spoils.applied_to_sample()).is_ok());
    }
}

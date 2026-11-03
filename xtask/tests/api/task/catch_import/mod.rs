mod catch_section;
mod day_file;

use std::collections::BTreeMap;

use guide::TripSource;
use guide::catch_words::{FeatureName, Readings};
use xtask::task::catch_import::CatchImport;

const TRIP: &str = "[trip]\nname = \"T\"\n\n[kana]\nunsourced = [\"A\"]\n\n[kana.readings.B]\nkana = \"い\"\nsource = \"s\"\n";

#[test]
fn without_a_kana_file_the_trip_keeps_the_readings_it_has() {
    let kept = CatchImport::kana_kept(TripSource::new(TRIP)).expect("the kana section");
    assert_eq!(
        kept.sourced_over(Readings::default()),
        Readings::new(BTreeMap::from([(
            FeatureName::new("B".to_owned()),
            FeatureName::new("い".to_owned())
        )]))
    );
}

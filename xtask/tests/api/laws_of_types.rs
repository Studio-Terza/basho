use laws::proptest::prelude::{Strategy, any};
use serde_json::json;
use xtask::task::catch_import::day_file::geo_json::GeoJson;
use xtask::task::gate::Commit;

laws::roundtrip!(
    geo_json_reads_back_from_its_text,
    any::<u32>().prop_map(|number| GeoJson::new(json!({ "id": number }))),
    |geo: &GeoJson| serde_json::to_string(geo).unwrap(),
    |text: &String| serde_json::from_str::<GeoJson>(text).ok(),
    Some,
);

laws::roundtrip!(
    commit_reads_back_from_its_text,
    "[0-9a-f]{40}".prop_map(|text| text.parse::<Commit>().unwrap()),
    |commit: &Commit| commit.to_string(),
    |text: &String| text.parse::<Commit>().ok(),
    Some,
);

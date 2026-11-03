use guide::catch_words::{FeatureName, Readings};
use guide::ruby_text::RubyText;
use std::collections::BTreeMap;

use guide::reading::Reading;
use guide::readout::day_catch::feature::Feature;

const UNREAD: &str = r#"{"id":"f1","kind":"river","name":"最上川","name_kana":null,"name_en":null,"ref":null,"geometry":{"type":"Point","coordinates":[140.0,38.0]}}"#;
const ROAD: &str = r#"{"id":"f2","kind":"road","name":"大石田畑線","name_kana":null,"name_en":null,"ref":"30;121","geometry":{"type":"LineString","coordinates":[[140.0,38.0],[140.01,38.0]]}}"#;
const READ: &str = "もがみがわ";

#[test]
fn a_name_without_a_reading_falls_back_to_words_the_walker_can_read() {
    let named = |text: &str| {
        serde_json::from_str::<Feature>(text).ok().map(|feature| {
            AsRef::<RubyText>::as_ref(&feature)
                .text(Reading::Written)
                .to_string()
        })
    };
    assert_eq!(named(UNREAD).as_deref(), Some("the river"));
    assert_eq!(named(ROAD).as_deref(), Some("Road 30 / 121"));
}

#[test]
fn a_sourced_reading_puts_furigana_over_the_kanji() {
    let readings = Readings::new(BTreeMap::from([(
        FeatureName::new("最上川".to_owned()),
        FeatureName::new(READ.to_owned()),
    )]));
    let feature = serde_json::from_str::<Feature>(UNREAD)
        .ok()
        .map(|feature| feature.read_again(&readings));
    assert_eq!(
        feature.map(|feature| AsRef::<RubyText>::as_ref(&feature)
            .text(Reading::Spoken)
            .to_string()),
        Some(READ.to_owned())
    );
}

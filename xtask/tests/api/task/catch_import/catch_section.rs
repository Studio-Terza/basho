use guide::catch_words::Readings;
use xtask::task::catch_import::catch_section::CatchSection;
use xtask::task::catch_import::day_file::DayFile;

const KANA: &str = r#"{"readings":{"A":{"kana":"あ","source":"s"},"B":{"kana":"い","source":""}}}"#;
const DAY: &str = r#"{"day":3,"features":[{"id":"f1","name_kana":null}],"legs":[]}"#;
const WRITTEN: &str = "[kana.readings.A]\nkana = \"あ\"\nsource = \"s\"\n\n[kana.readings.B]\nkana = \"い\"\nsource = \"\"\n\n[[catches]]\nday = 3\nlegs = []\n\n[[catches.features]]\nid = \"f1\"\n";

#[test]
fn the_section_drops_nulls_and_keeps_the_readings_before_the_days() {
    let kana: DayFile = KANA.parse().expect("kana");
    let day: DayFile = DAY.parse().expect("a day");
    assert_eq!(kana.sourced_over(Readings::default()).len(), 1);
    assert_eq!(
        CatchSection::new(kana, vec![day])
            .text()
            .expect("toml")
            .to_string(),
        WRITTEN
    );
}

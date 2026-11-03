use guide::readout::day_catch::leg_catch::catch_note::CatchNote;

const UNREAD: &str = r#"{"feature":"f1","ste":"The 仙石線 railway is north."}"#;
const FROM_THE_LINE: &str =
    r#"{"feature":"f1","ste":"The river is 200 m north of this leg. Go north to find it."}"#;

#[test]
fn a_note_with_kanji_without_a_reading_or_a_direction_from_the_line_fails_to_load() {
    assert!(serde_json::from_str::<CatchNote>(UNREAD).is_err());
    assert!(serde_json::from_str::<CatchNote>(FROM_THE_LINE).is_err());
}

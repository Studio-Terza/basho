use std::collections::BTreeMap;

use guide::readout::recovery::catch_words::{FeatureName, Readings};
use xtask::task::catch_import::day_file::note_text::NoteText;

const NOTE: &str = "The 最上川 is east. 上川 is west.";
const ANNOTATED: &str = "The {最上川|もがみがわ} is east. {上川|かみかわ} is west.";
const READ_ALREADY: &str = "It ends at {本塩釜|ほんしおがま} station.";

#[test]
fn longer_names_are_read_first_and_never_inside_another_reading() {
    let readings = Readings::new(BTreeMap::from([
        (
            FeatureName::new("上川".to_owned()),
            FeatureName::new("かみかわ".to_owned()),
        ),
        (
            FeatureName::new("最上川".to_owned()),
            FeatureName::new("もがみがわ".to_owned()),
        ),
    ]));
    assert_eq!(
        NoteText::from(NOTE).annotated(&readings).to_string(),
        ANNOTATED
    );
}

#[test]
fn a_reading_already_in_the_note_is_not_read_again_inside_itself() {
    let readings = Readings::new(BTreeMap::from([
        (
            FeatureName::new("塩釜".to_owned()),
            FeatureName::new("しおがま".to_owned()),
        ),
        (
            FeatureName::new("本塩釜".to_owned()),
            FeatureName::new("ほんしおがま".to_owned()),
        ),
    ]));
    assert_eq!(
        NoteText::from(READ_ALREADY)
            .annotated(&readings)
            .to_string(),
        READ_ALREADY
    );
}

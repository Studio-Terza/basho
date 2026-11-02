use std::collections::BTreeMap;

use guide::readout::recovery::catch_words::{FeatureName, Readings};
use xtask::task::catch_import::day_file::note_text::NoteText;

struct Note;

impl Note {
    fn annotated(text: &&str) -> String {
        let readings = Readings::new(
            [
                ("上川", "かみかわ"),
                ("最上川", "もがみがわ"),
                ("塩釜", "しおがま"),
                ("本塩釜", "ほんしおがま"),
            ]
            .map(|(name, reading)| {
                (
                    FeatureName::new(name.to_owned()),
                    FeatureName::new(reading.to_owned()),
                )
            })
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
        );
        NoteText::from(*text).annotated(&readings).to_string()
    }
}

laws::table!(
    longer_names_are_read_first_and_never_inside_a_reading_already_in_the_note,
    Note::annotated,
    [
        "The 最上川 is east. 上川 is west." => "The {最上川|もがみがわ} is east. {上川|かみかわ} is west.",
        "It ends at {本塩釜|ほんしおがま} station." => "It ends at {本塩釜|ほんしおがま} station.",
    ],
);

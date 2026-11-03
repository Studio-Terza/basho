use guide::TripSource;
use guide::content::Content;
use guide::itinerary::Arc;
use guide::ruby_text::RubyText;
use guide::words::Spelling;
use guide::{DraftPart, KnownNames};

const SAMPLE: &str = include_str!("../../../../../../data/trips/sample.toml");
const STOP_HERE: &str = "reach = [\"the inn\"]\n";
const NO_EXCERPTS: &str = "excerpts = []\n";
const NO_DISAGREEMENTS: &str = "disagreements = [{ topic = [\"The \", { entry = \"Mill Inn\" }, \" visit\"], detail = [\"One account has the walkers at the \", { entry = \"Mill Inn\" }, \" on the first night. Another has them at \", { entry = \"Eastfield\" }, \".\"] }]\n";
const LEAF: &str = "[\"By Eastfield.\"]";
const CLEAN: &str = "[\"Clean.\"]";
const QUOTE: &str = "[{ quote = \"古\" }]";
const EVIDENCE: &str = "evidence = { unverified = [\"Clean.\"] }";
const LOCATED: &str = "located = { url = \"https://example.org\", checked = \"2026-10-31\" }";
const NAMED: &str = "the words name Eastfield";

struct Planted;

impl Planted {
    fn on_the_leg(line: &str) -> String {
        SAMPLE.replacen(STOP_HERE, &format!("{line}\n{STOP_HERE}"), 1)
    }

    fn card(purpose: &str, japanese: &str, english: &str) -> String {
        Self::on_the_leg(&format!(
            "cards = [{{ purpose = {purpose}, japanese = {japanese}, english = {english} }}]"
        ))
    }

    fn branch(spot: &str, trigger: &str, main: &str, backup: &str) -> String {
        Self::on_the_leg(&format!(
            "branches = [{{ where = {spot}, trigger = {trigger}, main = {main}, backup = {backup} }}]"
        ))
    }

    fn stop(name: &str, detail: &str) -> String {
        Self::on_the_leg(&format!(
            "along = [{{ km = 0.1, kind = \"landmark\", name = {name}, detail = {{ text = {detail}, {EVIDENCE} }}, at = {{ lat = 38.0, lon = -77.996 }}, {LOCATED} }}]"
        ))
    }

    fn excerpt(translation: &str, provenance: &str) -> String {
        SAMPLE.replacen(
        NO_EXCERPTS,
        &format!(
            "excerpts = [{{ year = 1689, author = \"A\", work = \"W\", translator = \"ours\", translation = {translation}, original = {QUOTE}, provenance = {provenance}, source = {{ url = \"https://example.org\", checked = \"2026-10-31\" }} }}]\n"
        ),
        1,
    )
    }

    fn disagreement(topic: &str, detail: &str) -> String {
        SAMPLE.replacen(
            NO_DISAGREEMENTS,
            &format!("disagreements = [{{ topic = {topic}, detail = {detail} }}]\n"),
            1,
        )
    }

    fn replanted(text: &str) -> String {
        SAMPLE.replacen(text, "By Eastfield.", 1)
    }
}

#[test]
fn a_known_name_in_the_words_of_any_field_is_named_at_load() {
    let planted = [
        Planted::replanted("The stream is on your right."),
        Planted::replanted("Turn left."),
        Planted::replanted("A bridge."),
        Planted::replanted("The lane starts at the bridge."),
        Planted::replanted("A quiet lane."),
        Planted::replanted("Sleep here."),
        Planted::replanted("Walk a short sample day."),
        Planted::replanted("A made-up village for the sample trip."),
        Planted::replanted("The field to the east."),
        Planted::replanted("Made up for this sample."),
        Planted::replanted("A low ridge between two streams."),
        Planted::replanted("This place is not real."),
        Planted::replanted("Ridge Road"),
        Planted::replanted("the bridge"),
        Planted::on_the_leg(&format!("notes = [{{ text = {LEAF}, {EVIDENCE} }}]")),
        Planted::card(LEAF, CLEAN, CLEAN),
        Planted::card(CLEAN, LEAF, CLEAN),
        Planted::card(CLEAN, CLEAN, LEAF),
        Planted::branch(LEAF, CLEAN, CLEAN, CLEAN),
        Planted::branch(CLEAN, LEAF, CLEAN, CLEAN),
        Planted::branch(CLEAN, CLEAN, LEAF, CLEAN),
        Planted::branch(CLEAN, CLEAN, CLEAN, LEAF),
        Planted::stop(LEAF, CLEAN),
        Planted::stop(CLEAN, LEAF),
        Planted::excerpt(LEAF, CLEAN),
        Planted::excerpt(CLEAN, LEAF),
        Planted::disagreement(LEAF, CLEAN),
        Planted::disagreement(CLEAN, LEAF),
    ];
    let quiet: Vec<&String> = planted
        .iter()
        .filter(|text| {
            !TripSource::new(text)
                .findings()
                .iter()
                .any(|finding| finding.to_string().contains(NAMED))
        })
        .collect();
    assert!(quiet.is_empty(), "{} leaves passed", quiet.len());
}

#[test]
fn a_part_naming_nothing_known_fails_at_load() {
    let spoiled = SAMPLE.replacen("{ entry = \"Mill Inn\" }", "{ entry = \"Mill Barn\" }", 1);
    let placed = SAMPLE.replacen("\"The field to the east.\"", "{ place = \"Nowhere\" }", 1);
    let findings = [spoiled, placed].map(|text| {
        TripSource::new(&text)
            .findings()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
    });
    assert!(
        findings[0]
            .iter()
            .any(|finding| finding.contains("Mill Barn"))
    );
    assert!(
        findings[1]
            .iter()
            .any(|finding| finding.contains("Nowhere"))
    );
}

#[test]
fn a_generated_note_is_cut_into_parts() {
    let trip = TripSource::new(SAMPLE).trip().expect("the sample loads");
    let names = KnownNames::new(
        trip.itinerary(),
        trip.places().places(),
        trip.overview().landmarks(),
    );
    let note = RubyText::try_from("At 9:00, 250 m past Eastfield.".to_owned()).expect("a note");
    let words = |text: &str| DraftPart::Words(Spelling::new(text.to_owned()));
    assert_eq!(
        names.drafted(&note),
        [
            words("At "),
            DraftPart::Unsure(Spelling::new("9:00".to_owned())),
            words(", "),
            DraftPart::Length(Spelling::new("250 m".to_owned())),
            words(" past "),
            DraftPart::Mention(Spelling::new("Eastfield".to_owned())),
            words("."),
        ]
    );
}

#[test]
fn a_whole_stretch_of_content_lists_what_it_mentions() {
    let trip = TripSource::new(SAMPLE).trip().expect("the sample loads");
    let contents: Vec<&Content> = trip.itinerary().arcs().iter().map(Arc::title).collect();
    assert_eq!(
        contents
            .iter()
            .map(|text| text.mentions().count())
            .sum::<usize>(),
        1
    );
}

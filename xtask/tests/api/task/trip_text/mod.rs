use guide::TripSource;
use guide::words::Label;
use xtask::task::trip_text::SectionText;

const TEXT: &str = "[trip]\n\n[kana]\nold\n\n[overview]\ncoast\n\n[[maps]]\nday = 0\n";
const NEW_KANA: &str = "[trip]\n\n[kana]\nnew\n\n[overview]\ncoast\n\n[[maps]]\nday = 0\n";
const NEW_MAP: &str = "[trip]\n\n[kana]\nold\n\n[overview]\ncoast\n\n[[maps]]\nday = 1\n";

#[test]
fn a_closed_section_keeps_what_follows_it() {
    let section = SectionText::new("[kana]\nnew\n".to_owned());
    let swapped = section.spliced_into(
        TripSource::new(TEXT),
        Label::new("\n[kana"),
        Some(Label::new("\n[overview]\n")),
    );
    assert_eq!(swapped.to_string(), NEW_KANA);
}

#[test]
fn an_open_section_replaces_the_rest_of_the_file() {
    let section = SectionText::new("[[maps]]\nday = 1\n".to_owned());
    let swapped = section.spliced_into(TripSource::new(TEXT), Label::new("\n[[maps]]\n"), None);
    assert_eq!(swapped.to_string(), NEW_MAP);
}

#[test]
fn a_missing_section_is_added_at_the_end() {
    let section = SectionText::new("[[maps]]\n".to_owned());
    let added = section.spliced_into(
        TripSource::new("[trip]\n"),
        Label::new("\n[[maps]]\n"),
        None,
    );
    assert_eq!(added.to_string(), "[trip]\n\n[[maps]]\n");
}

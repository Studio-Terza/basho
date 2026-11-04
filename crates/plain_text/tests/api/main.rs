use laws::proptest::prelude::Strategy;
use plain_text::{NotPlain, PlainText};

const TEXT: &str = "[ -~\n\r\t±·]{0,12}";

laws::roundtrip!(
    plain_text_reads_back_from_its_display,
    TEXT.prop_map(|text| text.parse::<PlainText>().unwrap()),
    PlainText::to_string,
    |shown: &String| shown.parse::<PlainText>().ok(),
    Some,
);

#[test]
fn text_with_newlines_and_japanese_is_plain() {
    let text = "{仙台|せんだい}\r\n\tline";
    let shown = text.parse::<PlainText>().unwrap().to_string();
    assert_eq!(
        shown.split(['\r', '\n', '\t']).collect::<Vec<&str>>(),
        ["{仙台|せんだい}", "", "", "line"]
    );
    assert_eq!(
        shown
            .chars()
            .filter(char::is_ascii_control)
            .collect::<Vec<char>>(),
        ['\r', '\n', '\t']
    );
}

#[test]
fn a_control_character_is_refused() {
    assert_eq!("a\u{0}b".parse::<PlainText>(), Err(NotPlain));
    assert_eq!(PlainText::try_from("a\u{1b}[0m".to_owned()), Err(NotPlain));
}

#[test]
fn plain_text_reads_from_json_only_when_plain() {
    let read = |json: &str| serde_json::from_str::<PlainText>(json);
    let name: String = read("\"name\"").unwrap().into();
    assert_eq!(name, "name");
    assert!(read("\"a\\u0000b\"").is_err());
}

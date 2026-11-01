use guide::place::RubyText;

#[test]
fn annotated_names_split_into_base_and_reading_around_plain_text() {
    let text = RubyText::try_from("{平泉|ひらいずみ} and {衣川|ころもがわ}".to_owned());
    assert_eq!(text.map(|text| text.segments().len()).ok(), Some(3));
}

#[test]
fn kanji_without_a_reading_is_rejected() {
    assert!(RubyText::try_from("平泉".to_owned()).is_err());
}

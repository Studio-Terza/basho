mod load;

use guide::content::{Content, Part, TypedPart};
use guide::evidence::EvidenceKind;
use guide::reading::Reading;
use serde::Deserialize;

#[derive(Deserialize)]
struct Holder {
    text: Content,
}

struct Reader;

impl Reader {
    fn read(text: &str) -> Result<Content, toml::de::Error> {
        toml::from_str::<Holder>(text).map(|holder| holder.text)
    }

    fn written(text: &str) -> String {
        Self::read(text).map_or_else(
            |error| error.to_string(),
            |content| content.text(Reading::Written).to_string(),
        )
    }
}

#[test]
fn parts_show_as_their_author_wrote_them() {
    let shown = [
        (
            r#"text = ["Open ", { plan = "5:00" }, " to ", { plan = "24:00" }]"#,
            "Open 5:00 to 24:00",
        ),
        (r#"text = [{ plan = "08:10" }]"#, "08:10"),
        (
            r#"text = [{ distance = "1,200 m" }, " and ", { distance = "1.5 km" }]"#,
            "1,200 m and 1.5 km",
        ),
        (
            r#"text = [{ span = "5-minute" }, " or ", { span = "2 hours" }]"#,
            "5-minute or 2 hours",
        ),
        (
            r#"text = [{ price = "¥1,000" }, ", ", { train = "Hayabusa 45" }]"#,
            "¥1,000, Hayabusa 45",
        ),
        (
            r#"text = [{ place = "{仙台|せんだい}" }, " ", { entry = "{塩竈|しおがま}" }]"#,
            "仙台 塩竈",
        ),
        (r#"text = [{ quote = "五月雨の空" }]"#, "五月雨の空"),
        (
            r#"text = [{ fact = { at = "12:44", source = "train", evidence = { unverified = ["why"] } } }]"#,
            "12:44",
        ),
    ];
    assert!(
        shown
            .iter()
            .all(|(text, wanted)| Reader::written(text) == *wanted)
    );
}

#[test]
fn a_time_that_may_be_either_kind_carries_a_mark() {
    assert_eq!(
        Reader::written(r#"text = ["At ", { unknown = "15:03" }, "."]"#),
        "At 15:03?."
    );
}

#[test]
fn neighbouring_words_join_into_one_stretch_of_text() {
    let content = Reader::read(r#"text = ["a ", { plan = "9:00" }, " b {塩竈|しおがま}"]"#);
    assert_eq!(
        content.map(|content| content.segments().len()).ok(),
        Some(2)
    );
}

#[test]
fn a_mention_is_listed_and_other_parts_are_not() {
    let content =
        Reader::read(r#"text = ["to ", { entry = "Oakford" }, ", ", { place = "Mill" }]"#);
    assert_eq!(
        content.map(|content| content.mentions().count()).ok(),
        Some(1)
    );
}

#[test]
fn words_refuse_what_has_a_part_of_its_own() {
    let refused = [
        r#"text = ["Leave at 9:00."]"#,
        r#"text = ["Leave at 24:00"]"#,
        r#"text = ["Walk 450 m to the bridge."]"#,
        r#"text = ["Walk 1,200 m."]"#,
        r#"text = ["About 3km on."]"#,
        r#"text = ["平泉"]"#,
        r#"text = "A bare string."#,
        r#"text = [{ plan = "25:00" }]"#,
        r#"text = [{ plan = "9:60" }]"#,
        r#"text = [{ distance = "12 miles" }]"#,
        r#"text = [{ span = "12 km" }]"#,
        r#"text = [{ price = "1,000" }]"#,
        r#"text = [{ train = "Hikari 3" }]"#,
        r#"text = [{ shrug = "x" }]"#,
    ];
    assert!(refused.iter().all(|text| Reader::read(text).is_err()));
}

#[test]
fn words_keep_what_only_looks_typed() {
    let kept = [
        r#"text = ["Route 45 at 10 miles"]"#,
        r#"text = ["Yamabiko 145, car 6 and 3 monks"]"#,
        r#"text = ["a ratio of 1:25000"]"#,
        r#"text = ["5 min, ¥500"]"#,
        r#"text = ["see A5 m"]"#,
    ];
    assert!(kept.iter().all(|text| Reader::read(text).is_ok()));
}

#[derive(Debug, PartialEq)]
struct Stated(Option<EvidenceKind>);

impl Stated {
    fn of(part: &Part) -> Option<Self> {
        match part {
            Part::Typed(TypedPart::Stated(fact)) => Some(Self(fact.evidence())),
            Part::Words(_) | Part::Typed(_) => None,
        }
    }
}

#[test]
fn a_stated_fact_keeps_the_kind_of_its_evidence() {
    let content = Reader::read(
        r#"text = [{ fact = { at = "12:44", source = "train", evidence = { unverified = ["why"] } } }]"#,
    )
    .expect("the fact reads");
    let kinds: Vec<Stated> = content.parts().iter().filter_map(Stated::of).collect();
    assert_eq!(kinds, [Stated(Some(EvidenceKind::Unverified))]);
}

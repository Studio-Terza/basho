use std::sync::LazyLock;

use guide::Trip;
use sha1_smol::Sha1;

use day_print::DaysHtml;

const SAMPLE: &str = include_str!("../../../data/trips/sample.toml");
const GOLDEN: &str = "c3065e389bdee78f98cdf6d32a3d61f3d91be45e";
const HANDRAIL: &str = "The stream is on your right.";
const MARKUP: &str = "<a & b>";
const ESCAPED: &str = "&lt;a &amp; b&gt;";
const NO_PATH_WORDS: &str = "<b>⚠ No mapped path</b>";
const NO_PATH_LINE: &str = "class=\"no-mapped-path\"";
const ROW: &str = "<tr>";
const TRAIN_ROW: &str = "<b>Train · ";
const CELL_THEN_PLATFORM: &str = "<td><b>Platform ";
const UNVERIFIED_MARK: &str = " <span class=\"unverified\">?</span>";

static BUNDLED: LazyLock<Trip> = LazyLock::new(|| SAMPLE.parse::<Trip>().expect("the trip loads"));

#[test]
fn the_existing_trip_prints_as_it_did() {
    let printed = DaysHtml::new(BUNDLED.itinerary()).to_string();
    assert_eq!(Sha1::from(printed).hexdigest(), GOLDEN);
}

#[test]
fn a_printed_train_row_opens_with_its_board_facts_and_marks_them_if_unverified() {
    let train = SAMPLE
        .lines()
        .find(|line| line.starts_with("mode = { train"))
        .expect("the trip has a train");
    let unverified = train
        .rsplit_once(", evidence = ")
        .map(|(board, _)| format!("{board}, evidence = {{ unverified = [\"Made up.\"] }} }} }}"))
        .expect("a train has evidence");
    let trip = SAMPLE
        .replacen(train, &unverified, 1)
        .parse::<Trip>()
        .expect("the trip loads");
    let printed = DaysHtml::new(trip.itinerary()).to_string();
    let rows: Vec<&str> = printed
        .split(ROW)
        .filter(|row| row.contains(TRAIN_ROW))
        .collect();
    assert_ne!(rows, Vec::<&str>::new());
    assert!(rows.iter().all(|row| row.contains(CELL_THEN_PLATFORM)));
    assert!(printed.contains(&format!("{UNVERIFIED_MARK}<br>")));
}

#[test]
fn a_no_mapped_path_prints_a_warning_and_a_broken_line() {
    let trip = SAMPLE.parse::<Trip>().expect("the trip loads");
    let printed = DaysHtml::new(trip.itinerary()).to_string();
    assert!(printed.contains(NO_PATH_WORDS));
    assert!(printed.contains(NO_PATH_LINE));
}

#[test]
fn markup_characters_are_escaped() {
    let trip = SAMPLE
        .replace(HANDRAIL, MARKUP)
        .parse::<Trip>()
        .expect("the trip loads");
    let printed = DaysHtml::new(trip.itinerary()).to_string();
    assert!(printed.contains(ESCAPED));
    assert_eq!(printed.matches(MARKUP).count(), 0);
}

#[test]
fn an_unverified_night_and_time_print_with_a_mark_that_needs_no_colour() {
    let trip = SAMPLE
        .replacen(
            "start = { plan = \"09:00\" }",
            "start = { fact = \"09:00\", source = \"boat\", evidence = { unverified = [\"Made up.\"] } }",
            1,
        )
        .parse::<Trip>()
        .expect("the trip loads");
    let printed = DaysHtml::new(trip.itinerary()).to_string();
    assert!(printed.contains(&format!("Mill Inn (Mill Inn){UNVERIFIED_MARK}")));
    assert!(printed.contains("9:00 boat ?"));
}

#[test]
fn a_walk_without_a_recorded_reason_prints_that_in_words() {
    let trip = SAMPLE.parse::<Trip>().expect("the trip loads");
    let printed = DaysHtml::new(trip.itinerary()).to_string();
    assert!(printed.contains("<b>Why this way:</b> No reason recorded"));
}

#[test]
fn a_recorded_reason_prints_its_kind_and_what_it_refers_to() {
    let recorded = SAMPLE.replacen(
        "mode = { walk = { reason = \"none_recorded\" } }",
        "mode = { walk = { reason = { recorded = { kind = \"quieter\", refers = { entry = \"Eastfield\" }, evidence = { unverified = [\"Stated in the leg text.\"] } } } } }",
        1,
    );
    let trip = recorded.parse::<Trip>().expect("the trip loads");
    let printed = DaysHtml::new(trip.itinerary()).to_string();
    assert!(printed.contains("<b>Why this way:</b> Quieter · Eastfield"));
}

use crate::fixture::Fixture;
use guide::FactSource;
use guide::Trip;
use guide::classed::Classed;
use guide::clock::{ClockTime, Minutes};
use guide::end::End;
use guide::evidence::Evidence;
use guide::guidance::timeline::LiveGap;
use guide::itinerary::Day;
use guide::times::{Basis, FactTime, ListedTime, LiveTime, PlanTime, ShownTime};
use guide::words::Spelling;

const APPROXIMATE: char = '≈';
const SAMPLE: &str = include_str!("../../../../../data/trips/sample.toml");
const MARKED: &str = "start = { plan = \"09:00\" }";
const SOURCES: [FactSource; 13] = [
    FactSource::TrainTimes,
    FactSource::BusTimes,
    FactSource::BoatTimes,
    FactSource::TourTimes,
    FactSource::Booked,
    FactSource::Closes,
    FactSource::Entry,
    FactSource::LastOrder,
    FactSource::CheckIn,
    FactSource::Lands,
    FactSource::AtSunset,
    FactSource::Gps,
    FactSource::Unknown,
];

struct Written;

impl Written {
    fn clock(text: &str) -> ClockTime {
        ClockTime::try_from(text.to_owned()).unwrap()
    }

    fn word(text: &str) -> Spelling {
        Spelling::new(text.to_owned())
    }

    fn evidence() -> Evidence {
        toml::from_str("source = { url = \"https://example.org\", checked = \"2026-10-31\" }")
            .unwrap()
    }

    fn fact(text: &str, source: FactSource) -> FactTime {
        FactTime::new(Self::clock(text), source, &Self::evidence())
    }

    fn plan(text: &str) -> ListedTime {
        ListedTime::Planned(PlanTime::new(Self::clock(text)))
    }
}

#[test]
fn a_fact_reads_with_its_one_source_word() {
    let train = Written::fact("12:44", FactSource::TrainTimes);
    let closes = Written::fact("17:00", FactSource::Closes);
    assert_eq!(train.to_string(), "12:44 train");
    assert_eq!(closes.to_string(), "closes 17:00");
}

#[test]
fn a_fact_with_no_known_source_says_so() {
    let unknown = Written::fact("17:41", FactSource::Unknown);
    assert_eq!(unknown.to_string(), "17:41 source unknown");
}

#[test]
fn a_plan_reads_as_a_plan() {
    assert_eq!(
        PlanTime::new(Written::clock("13:15")).to_string(),
        "plan 13:15"
    );
}

#[test]
fn a_live_time_shows_its_basis_inline() {
    let left = LiveTime::new(Written::clock("15:10"), Basis::PlanLeft(Minutes::new(34)));
    assert_eq!(left.to_string(), "≈ 15:10 · 34 min of plan left");
}

#[test]
fn a_live_gap_shows_its_basis_on_both_sides_of_zero() {
    let basis = Basis::PlanLeft(Minutes::new(34));
    assert_eq!(
        LiveGap::new(Minutes::new(-8), basis).to_string(),
        "8 min short · 34 min of plan left"
    );
    assert_eq!(
        LiveGap::new(Minutes::new(8), basis).to_string(),
        "8 min to spare · 34 min of plan left"
    );
}

#[test]
fn facts_and_plans_never_carry_the_approximate_mark() {
    let noon = Written::clock("12:00");
    let marked: Vec<String> = SOURCES
        .map(|source| FactTime::new(noon, source, &Written::evidence()).to_string())
        .into_iter()
        .filter(|fact| fact.contains(APPROXIMATE))
        .collect();
    assert_eq!(marked, Vec::<String>::new());
    assert_eq!(PlanTime::new(noon).to_string().find(APPROXIMATE), None);
}

#[test]
fn every_time_on_every_bundled_leg_is_free_of_the_approximate_mark() {
    let trip = Fixture::bundled().unwrap();
    let marked: Vec<String> = trip
        .days()
        .iter()
        .flat_map(Day::legs)
        .flat_map(|leg| [leg.listed(End::Start), leg.listed(End::Finish)])
        .map(|time| time.to_string())
        .filter(|time| time.contains(APPROXIMATE))
        .collect();
    assert_eq!(marked, Vec::<String>::new());
}

#[test]
fn a_time_on_plan_stays_listed_and_one_off_it_turns_live() {
    let due = Written::plan("13:15");
    assert_eq!(ShownTime::off_by(due, Minutes::ZERO), ShownTime::from(due));
    assert_eq!(
        ShownTime::off_by(due, Minutes::new(12)).to_string(),
        "≈ 13:27 · 12 min behind plan 13:15"
    );
    assert_eq!(
        ShownTime::off_by(due, Minutes::new(-5)).to_string(),
        "≈ 13:10 · 5 min ahead of plan 13:15"
    );
}

#[test]
fn the_three_kinds_draw_with_three_classes() {
    let noon = Written::clock("12:00");
    let classes = [
        ShownTime::from(FactTime::new(
            noon,
            FactSource::TrainTimes,
            &Written::evidence(),
        ))
        .class(),
        ShownTime::from(PlanTime::new(noon)).class(),
        ShownTime::from(LiveTime::new(noon, Basis::PlanTimes)).class(),
    ];
    assert_eq!(
        classes.map(|class| class.to_string()),
        ["time-fact", "time-plan", "time-live"]
    );
}

#[test]
fn a_word_the_time_already_says_is_not_said_twice() {
    let train = ListedTime::Stated(Written::fact("12:44", FactSource::TrainTimes));
    let unknown = ListedTime::Stated(Written::fact("17:41", FactSource::Unknown));
    assert_eq!(
        train.worded(Written::word("train")).to_string(),
        "12:44 train"
    );
    assert_eq!(
        unknown.worded(Written::word("train")).to_string(),
        "train 17:41 source unknown"
    );
    assert_eq!(
        Written::plan("08:30")
            .worded(Written::word("car"))
            .to_string(),
        "car plan 8:30"
    );
}

#[test]
fn a_mark_with_no_source_word_or_with_both_marks_fails_at_load() {
    let spoiled = [
        "start = { fact = \"09:00\" }",
        "start = { fact = \"09:00\", source = \"sunset\", evidence = { unverified = [\"Made up.\"] } }",
        "start = { fact = \"09:00\", source = \"gps\", evidence = { unverified = [\"Made up.\"] } }",
        "start = { fact = \"09:00\", source = \"a friend\", evidence = { unverified = [\"Made up.\"] } }",
        "start = { plan = \"09:00\", source = \"train\", evidence = { unverified = [\"Made up.\"] } }",
    ];
    assert!(
        spoiled
            .iter()
            .all(|mark| SAMPLE.replace(MARKED, mark).parse::<Trip>().is_err())
    );
}

#[test]
fn the_bundled_trip_marks_a_ride_by_its_source_or_says_it_is_a_plan() {
    let trip = Fixture::bundled().unwrap();
    let train = trip.days()[1].legs()[1].listed(End::Start);
    let bus_with_no_timetable = trip.days()[3].legs()[0].listed(End::Start);
    assert_eq!(train.to_string(), "9:30 train ?");
    assert_eq!(bus_with_no_timetable.to_string(), "plan 7:30");
}

use guide::Trip;
use guide::end::End;
use guide::itinerary::DayNumber;
use guide::ruby_text::RubyText;
use guide::words::Slug;
use wiki::entry::{Entry, EntryKind, Heading, Opening, Row, Section, StopRow};
use wiki::{EntryId, EntryKey, Wiki};

const SAMPLE: &str = include_str!("../../../../data/trips/sample.toml");
const FIRST_LEG_END: &str = "at = { lat = 38.0, lon = -77.996 }\n";
const SECOND_LEG_END: &str = "at = { lat = 38.0, lon = -77.988 }\n\n[[days]]";
const SHRINE: &str = "Hill Shrine";
const BAKERY: &str = r#"
[[days.legs.along]]
km = 0.1
kind = "shop"
name = [{ entry = "Oak Bakery" }]
at = { lat = 38.0, lon = -77.9945 }
located = { url = "https://example.org", checked = "2026-10-31" }
"#;
const LATE_CAFE: &str = r#"
[[days.legs.along]]
km = 0.2
kind = "food"
name = [{ entry = "Ridge Cafe" }]
expected = { plan = "09:03" }
dwell = 1
cutoff = { kind = "closes", at = { fact = "09:10", source = "closes", evidence = { unverified = ["This place is not real."] } } }
detail = { text = ["Across from ", { entry = "Oak Bakery" }, "."], evidence = { unverified = ["This place is not real."] } }
at = { lat = 38.0, lon = -77.9935 }
located = { url = "https://example.org", checked = "2026-10-31" }
"#;

struct Sample {
    wiki: Wiki,
}

impl Sample {
    fn new() -> Sample {
        let text = SAMPLE
            .replace(
                FIRST_LEG_END,
                &format!("{FIRST_LEG_END}{BAKERY}{LATE_CAFE}"),
            )
            .replace(
                SECOND_LEG_END,
                &SECOND_LEG_END.replace("\n\n", &format!("\n{BAKERY}\n")),
            );
        let trip: Trip = text.parse().expect("the sample with stops loads");
        Sample {
            wiki: Wiki::new(
                trip.itinerary(),
                trip.places().places(),
                trip.overview().landmarks(),
            ),
        }
    }

    fn of_kind(&self, kind: EntryKind) -> Vec<EntryId> {
        self.wiki
            .pages()
            .into_iter()
            .filter(|id| self.wiki.entry(*id).map(Entry::kind) == Some(kind))
            .collect()
    }

    fn rows(&self, id: EntryId, heading: Heading) -> &[Row] {
        self.wiki
            .entry(id)
            .and_then(|entry| {
                entry
                    .sections()
                    .iter()
                    .find(|section| section.heading() == heading)
            })
            .map_or(&[], |section| section.rows())
    }
}

#[test]
fn a_stop_passed_twice_is_one_entry_with_both_visits() {
    let sample = Sample::new();
    let shops = sample.of_kind(EntryKind::Shop);
    assert_eq!(shops.len(), 1);
    assert_eq!(sample.rows(shops[0], Heading::OnTheRoute).len(), 2);
}

#[test]
fn a_mention_in_a_detail_links_and_the_named_entry_lists_who_linked_it() {
    let sample = Sample::new();
    let wiki = &sample.wiki;
    let shop = sample.of_kind(EntryKind::Shop)[0];
    let cafe = sample.of_kind(EntryKind::Meal)[0];
    let detail: Vec<EntryId> = sample
        .rows(cafe, Heading::OnTheRoute)
        .iter()
        .flat_map(Row::texts)
        .flat_map(|text| wiki.names().runs(text, text.shown(), None))
        .filter_map(|run| run.entry())
        .collect();
    assert!(detail.contains(&shop));
    assert!(
        sample
            .rows(shop, Heading::LinkedHere)
            .iter()
            .any(|row| row.links() == [cafe])
    );
}

#[test]
fn opening_is_judged_at_the_arrival_and_empty_sections_are_left_out() {
    let sample = Sample::new();
    let wiki = &sample.wiki;
    let opening = |id: EntryId, heading: Heading| {
        sample
            .rows(id, heading)
            .iter()
            .find_map(|row| row.stop().and_then(StopRow::opening))
    };
    let cafe = sample.of_kind(EntryKind::Meal)[0];
    let evening = sample.of_kind(EntryKind::Evening)[0];
    assert_eq!(opening(cafe, Heading::OnTheRoute), Some(Opening::Open));
    assert_eq!(opening(evening, Heading::Dinner), Some(Opening::Closed));
    let empty = wiki
        .pages()
        .into_iter()
        .filter_map(|id| wiki.entry(id))
        .flat_map(Entry::sections)
        .filter(|section| section.rows().is_empty())
        .count();
    assert_eq!(empty, 0);
    assert!(wiki.find(&EntryKey::Shops).is_some());
}

#[test]
fn a_thing_nobody_can_reach_by_its_name_has_no_page_beside_the_place_of_that_name() {
    let trip: Trip = include_str!("../../../../data/trips/sample.toml")
        .parse()
        .expect("the built-in trip loads");
    let wiki = Wiki::new(
        trip.itinerary(),
        trip.places().places(),
        trip.overview().landmarks(),
    );
    let stranded: Vec<String> = wiki
        .pages()
        .into_iter()
        .filter_map(|id| wiki.entry(id).map(|entry| (id, entry)))
        .filter(|(_, entry)| entry.kind() == EntryKind::Thing)
        .filter(|(id, entry)| wiki.names().entry_named(entry.name()) != Some(*id))
        .map(|(_, entry)| format!("{:?}", entry.name()))
        .collect();
    assert_eq!(stranded, Vec::<String>::new());
}

#[test]
fn a_shrine_and_its_car_park_a_short_walk_apart_are_one_page_that_its_name_reaches() {
    let trip: Trip = SAMPLE.parse().expect("the built-in trip loads");
    let wiki = Wiki::new(
        trip.itinerary(),
        trip.places().places(),
        trip.overview().landmarks(),
    );
    let name = RubyText::try_from(SHRINE.to_owned()).unwrap();
    let page = wiki
        .names()
        .entry_named(&name)
        .and_then(|id| wiki.entry(id))
        .map(Entry::kind);
    assert_eq!(page, Some(EntryKind::Place));
}

#[test]
fn a_loop_goal_page_lists_its_sights_numbered_in_order_with_their_times() {
    let trip: Trip = SAMPLE.parse().expect("the built-in trip loads");
    let wiki = Wiki::new(
        trip.itinerary(),
        trip.places().places(),
        trip.overview().landmarks(),
    );
    let looped: Vec<Vec<String>> = trip
        .itinerary()
        .days()
        .iter()
        .flat_map(|day| {
            day.goals()
                .iter()
                .filter(|goal| !goal.purpose().stops().is_empty())
                .map(|goal| EntryKey::Goal {
                    day: day.number(),
                    first: goal.leg(End::Start),
                })
        })
        .filter_map(|key| wiki.find(&key))
        .map(|id| {
            wiki.entry(id)
                .into_iter()
                .flat_map(Entry::sections)
                .filter(|section| section.heading() == Heading::Loop)
                .flat_map(Section::rows)
                .filter_map(Row::looped)
                .map(|stop| format!("{} {}", stop.number(), stop.planned()))
                .collect()
        })
        .collect();
    assert_eq!(looped, [["1 plan 13:35", "2 plan 13:55", "3 plan 14:15"]]);
}

#[test]
fn a_name_in_the_topic_of_a_disagreement_links_back_from_the_named_entry() {
    let detail = "detail = [\"One account has the walkers at the \", { entry = \"Mill Inn\" }, \" on the first night. Another has them at \", { entry = \"Eastfield\" }, \".\"]";
    let text = SAMPLE.replacen(detail, "detail = [\"The accounts differ.\"]", 1);
    let trip: Trip = text.parse().expect("the sample with a plain detail loads");
    let wiki = Wiki::new(
        trip.itinerary(),
        trip.places().places(),
        trip.overview().landmarks(),
    );
    let inn = wiki
        .find(&EntryKey::Inn(DayNumber::new(1)))
        .expect("an inn");
    let place = wiki
        .find(&EntryKey::Place(Slug::new("eastfield".to_owned())))
        .expect("a place");
    let linked: Vec<EntryId> = wiki
        .entry(inn)
        .into_iter()
        .flat_map(Entry::sections)
        .filter(|section| section.heading() == Heading::LinkedHere)
        .flat_map(Section::rows)
        .flat_map(Row::links)
        .collect();
    assert_ne!(text, SAMPLE);
    assert!(linked.contains(&place));
}

use rayon::prelude::*;
use std::collections::BTreeSet;

use basho::app::screen::Screen;
use wiki::EntryKey;

use crate::page_checks::PageTree;
use crate::ui_check::{Drawing, Harness, Stage};

const JUMP: &str = "menu jump";

pub struct RenderChecks;

impl RenderChecks {
    const FIRST_TRIP_WORD: &str = Drawing::FIRST_NAME;
    const NO_PATH_WORDS: &str = "<strong>No mapped path</strong>";
    const UNVERIFIED: &str = "class=\"unverified\"";
    const INN_OPEN: &str = "Inn: <strong>";
    const INN_END: &str = "</strong>";

    pub fn problems(stages: &[Stage]) -> Vec<String> {
        Self::across(stages, PageTree::stage_problems)
    }

    pub fn second_trip_faults(stage: &Stage) -> Vec<String> {
        let page = Harness::page(stage);
        page.parse::<PageTree>()
            .unwrap_or_else(|never| match never {})
            .problems(&[])
            .into_iter()
            .chain(
                page.contains(Self::FIRST_TRIP_WORD)
                    .then(|| stage.label().to_owned()),
            )
            .collect()
    }

    pub fn no_mapped_path_parts(stage: &Stage) -> Vec<String> {
        let page = Harness::page(stage);
        [Self::NO_PATH_WORDS]
            .into_iter()
            .filter(|part| page.contains(part))
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn page_of(stage: &Stage) -> Vec<String> {
        vec![Harness::page(stage)]
    }

    pub fn inns(stage: &Stage) -> Vec<String> {
        Harness::page(stage)
            .split(Self::INN_OPEN)
            .skip(1)
            .filter_map(|rest| rest.split(Self::INN_END).next())
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn across(stages: &[Stage], read: fn(&Stage) -> Vec<String>) -> Vec<String> {
        stages.par_iter().flat_map_iter(read).collect()
    }
}

#[test]
fn a_second_trip_renders_every_screen_with_nothing_of_the_first() {
    let stages: Vec<Stage> = Stage::all(&Drawing::Second.itinerary(), &Drawing::Second.wiki())
        .into_iter()
        .map(|stage| stage.drawn_with(Drawing::Second))
        .collect();
    let faults = RenderChecks::across(&stages, RenderChecks::second_trip_faults);
    assert!(!stages.is_empty());
    assert_eq!(faults, Vec::<String>::new());
}

#[test]
fn every_stage_renders_what_it_should() {
    let (itinerary, wiki) = (Drawing::BuiltIn.itinerary(), Drawing::BuiltIn.wiki());
    let problems = RenderChecks::problems(&Stage::all(&itinerary, &wiki));
    assert!(
        problems.is_empty(),
        "{} render problems:\n{}",
        problems.len(),
        problems.join("\n")
    );
}

#[test]
fn the_map_and_today_show_the_step_a_menu_jump_picked() {
    let (itinerary, wiki) = (Drawing::BuiltIn.itinerary(), Drawing::BuiltIn.wiki());
    let jumps: Vec<Stage> = Stage::all(&itinerary, &wiki)
        .into_iter()
        .filter(|stage| stage.label().contains(JUMP))
        .collect();
    let unmatched: Vec<&str> = jumps
        .iter()
        .filter(|stage| !stage.marks_its_focus(&Harness::page(stage)))
        .map(Stage::label)
        .collect();
    assert!(!jumps.is_empty(), "no menu-jump stages");
    assert!(
        unmatched.is_empty(),
        "the jumped-to step is not shown: {unmatched:#?}"
    );
}

#[test]
fn a_no_mapped_path_shows_in_words() {
    let stages: Vec<Stage> = Stage::all(&Drawing::Second.itinerary(), &Drawing::Second.wiki())
        .into_iter()
        .map(|stage| stage.drawn_with(Drawing::Second))
        .collect();
    let parts = RenderChecks::across(&stages, RenderChecks::no_mapped_path_parts);
    let shown: BTreeSet<&str> = parts.iter().map(String::as_str).collect();
    assert_eq!(shown, BTreeSet::from([RenderChecks::NO_PATH_WORDS]));
}

#[test]
fn an_unverified_inn_is_marked_where_it_is_named() {
    let stages = Stage::all(&Drawing::Second.itinerary(), &Drawing::Second.wiki());
    let inns: Vec<String> = stages
        .into_iter()
        .map(|stage| stage.drawn_with(Drawing::Second))
        .flat_map(|stage| RenderChecks::inns(&stage))
        .collect();
    assert_ne!(inns, Vec::<String>::new());
    assert!(
        inns.iter()
            .all(|inn| inn.contains(RenderChecks::UNVERIFIED))
    );
}

#[test]
fn a_mention_in_a_day_page_is_a_link_and_a_wiki_page_names_its_kind() {
    let itinerary = Drawing::BuiltIn.itinerary();
    let day = itinerary.days().first().expect("a day");
    let index = Drawing::BuiltIn.wiki().find(&EntryKey::Index);
    let today = Stage::on_screen(day, Screen::Today);
    let page = Stage::on_screen(day, index.map_or(Screen::Today, Screen::Wiki));
    assert!(Harness::page(&today).contains("class=\"link\""));
    assert!(Harness::page(&page).contains("class=\"wiki-kind\""));
}

#[test]
fn walking_cards_show_a_reason_or_say_in_words_that_none_is_recorded() {
    let itinerary = Drawing::BuiltIn.itinerary();
    let wiki = Drawing::BuiltIn.wiki();
    let pages = RenderChecks::across(&Stage::all(&itinerary, &wiki), RenderChecks::page_of);
    let showing = |words: &str| pages.iter().any(|page| page.contains(words));
    assert!(showing("<dt>WHY THIS WAY</dt>"));
    assert!(showing("<strong>No reason recorded</strong>"));
    assert!(showing("<strong>Point of interest</strong>"));
    assert!(showing("<strong>Quieter</strong>"));
}

use std::collections::BTreeMap;

use dioxus::prelude::*;
use guide::plane::{Pixels, PlanePoint};
use layout::MarkSize;
use map_plan::MapMark;
use map_plan::MarkAt;

#[test]
fn every_mark_looks_different_in_greyscale() {
    let clashes: Vec<String> = MapMark::ALL
        .iter()
        .fold(BTreeMap::new(), |mut looks, mark| {
            looks
                .entry(mark.look())
                .or_insert_with(Vec::new)
                .push(*mark);
            looks
        })
        .into_values()
        .filter(|marks| marks.len() > 1)
        .map(|marks| format!("{marks:?}"))
        .collect();
    assert!(
        clashes.is_empty(),
        "marks that look the same:\n{}",
        clashes.join("\n")
    );
}

#[test]
fn every_mark_is_listed_once() {
    let listed: Vec<MapMark> = MapMark::ALL
        .iter()
        .enumerate()
        .filter(|(index, mark)| MapMark::ALL[..*index].contains(mark))
        .map(|(_, mark)| *mark)
        .collect();
    assert!(listed.is_empty(), "listed twice: {listed:?}");
}

struct StarMark;

impl StarMark {
    fn drawn() -> Element {
        rsx! {
            svg { {MarkAt::new(MapMark::Star, PlanePoint::new(Pixels::new(50.0), Pixels::new(50.0)))} }
        }
    }
}

#[test]
fn a_star_mark_draws_a_corner_for_every_point_of_the_star() {
    let mut dom = VirtualDom::new(StarMark::drawn);
    dom.rebuild_in_place();
    let markup = dioxus_ssr::render(&dom);
    let corners = markup
        .split("points=\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .map_or(0, |points| points.split_whitespace().count());
    assert_eq!(corners, MarkSize::star_corners().len());
}

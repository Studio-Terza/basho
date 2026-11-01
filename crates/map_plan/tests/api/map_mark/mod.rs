use std::collections::BTreeMap;

use map_plan::map_mark::MapMark;

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

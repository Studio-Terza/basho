mod held_phone;

use std::fs;

use basho::app::session::trip_shelf::{PickedFile, TripInput};
use held_phone::HeldPhone;

const FAILED: &str = "The guide data failed to load";
const FILE_INPUT: &str = "trip-file";
const NOT_CHANGED: &str = "The current trip did not change.";

#[test]
fn a_good_file_replaces_the_trip_and_forgets_the_old_progress() {
    let home = tempfile::tempdir().unwrap();
    let progress = HeldPhone::progress_kept_under(home.path());
    let mut phone = HeldPhone::on_the_sample(home.path());
    let before = phone.page();
    phone.offer(&HeldPhone::later_trip());
    assert_ne!(phone.page(), before);
    assert!(phone.page_came_up_afresh().is_some());
    assert!(!progress.exists());
}

#[test]
fn a_bad_file_shows_why_and_leaves_the_trip_and_its_progress_alone() {
    let home = tempfile::tempdir().unwrap();
    let progress = HeldPhone::progress_kept_under(home.path());
    let mut phone = HeldPhone::on_the_sample(home.path());
    let before = phone.page();
    phone.offer(&HeldPhone::picked("broken.toml", Some("not a trip")));
    assert_eq!(phone.page(), before);
    let refusal = phone.refusal().expect("the file is refused");
    assert!(refusal.starts_with("broken.toml") && refusal.contains(NOT_CHANGED));
    assert!(progress.exists());
}

#[test]
fn a_file_that_is_not_text_is_refused_by_name() {
    let home = tempfile::tempdir().unwrap();
    let mut phone = HeldPhone::on_the_sample(home.path());
    phone.offer(&HeldPhone::picked("photo.heic", None));
    let refusal = phone.refusal().expect("the file is refused");
    assert!(refusal.starts_with("photo.heic") && refusal.contains(NOT_CHANGED));
}

#[test]
fn opening_the_shown_trip_again_keeps_its_progress_and_clears_an_old_refusal() {
    let home = tempfile::tempdir().unwrap();
    let progress = HeldPhone::progress_kept_under(home.path());
    let mut phone = HeldPhone::on_the_sample(home.path());
    phone.offer(&HeldPhone::picked("broken.toml", Some("not a trip")));
    phone.offer(&HeldPhone::sample());
    assert!(phone.refusal().is_none());
    assert!(progress.exists());
}

#[test]
fn the_chosen_trip_comes_back_at_the_next_launch_and_beats_the_documents_file() {
    let home = tempfile::tempdir().unwrap();
    let mut phone = HeldPhone::on_the_sample(home.path());
    phone.offer(&HeldPhone::later_trip());
    let chosen = phone.page();
    assert_eq!(HeldPhone::opened(home.path()).page(), chosen);
}

#[test]
fn the_built_in_trip_can_be_chosen_again() {
    let home = tempfile::tempdir().unwrap();
    let mut phone = HeldPhone::on_the_sample(home.path());
    let built_in = phone.page();
    phone.offer(&HeldPhone::later_trip());
    assert_ne!(phone.page(), built_in);
    phone.offer(&PickedFile::built_in());
    assert_eq!(phone.page(), built_in);
}

#[test]
fn a_chosen_trip_that_no_longer_loads_shows_why_and_offers_the_picker() {
    let home = tempfile::tempdir().unwrap();
    let chosen = TripInput::chosen_under(home.path());
    chosen.replaced("not a trip").unwrap();
    let mut phone = HeldPhone::opened(home.path());
    assert!(phone.page().contains(FAILED) && phone.page().contains(FILE_INPUT));
    phone.offer(&HeldPhone::sample());
    assert!(!phone.page().contains(FAILED));
}

#[test]
fn a_trip_the_phone_cannot_save_is_refused_and_the_trip_stays() {
    let home = tempfile::tempdir().unwrap();
    let mut phone = HeldPhone::on_the_sample(home.path());
    fs::write(home.path().join("Library"), "in the way").unwrap();
    let before = phone.page();
    phone.offer(&HeldPhone::later_trip());
    assert_eq!(phone.page(), before);
    let refusal = phone.refusal().expect("the file is refused");
    assert!(refusal.starts_with("later.toml") && refusal.contains(NOT_CHANGED));
}

#[test]
fn choosing_the_built_in_trip_follows_this_builds_copy_not_a_frozen_one() {
    let home = tempfile::tempdir().unwrap();
    let mut phone = HeldPhone::on_the_sample(home.path());
    phone.offer(&HeldPhone::later_trip());
    phone.offer(&PickedFile::built_in());
    let kept = fs::read_to_string(TripInput::chosen_under(home.path())).unwrap();
    assert_ne!(kept, TripInput::BUILT_IN);
    let opened = TripInput::read(None, Some(home.path().into())).unwrap();
    assert_eq!(opened.to_string(), TripInput::BUILT_IN);
}

#[test]
fn the_phone_is_told_a_toml_file_is_plain_text_so_its_picker_offers_one() {
    let manifest: toml::Table = include_str!("../../../Dioxus.toml").parse().unwrap();
    let plist = manifest["ios"]["raw"]["info_plist"].as_str().unwrap();
    assert!(plist.contains("<key>UTImportedTypeDeclarations</key>"));
    assert!(plist.contains("<string>public.plain-text</string>"));
    assert!(plist.contains("<string>toml</string>"));
}

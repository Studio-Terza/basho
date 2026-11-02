mod held_phone;

use basho::app::session::trip_shelf::{PickedFile, TripInput};
use held_phone::HeldPhone;
use storage_fake::MemoryStorage;

const FAILED: &str = "The guide data failed to load";
const FILE_INPUT: &str = "trip-file";
const NOT_CHANGED: &str = "The current trip did not change.";

#[test]
fn a_good_file_replaces_the_trip_and_forgets_the_old_progress() {
    let home = MemoryStorage::home();
    let progress = HeldPhone::progress_kept_under(&home);
    let mut phone = HeldPhone::on_the_sample(&home);
    let before = phone.page();
    phone.offer(&HeldPhone::later_trip());
    assert_ne!(phone.page(), before);
    assert!(phone.page_came_up_afresh().is_some());
    assert!(home.read(&progress).is_err());
}

#[test]
fn a_bad_file_shows_why_and_leaves_the_trip_and_its_progress_alone() {
    let home = MemoryStorage::home();
    let progress = HeldPhone::progress_kept_under(&home);
    let mut phone = HeldPhone::on_the_sample(&home);
    let before = phone.page();
    phone.offer(&HeldPhone::picked("broken.toml", Some("not a trip")));
    assert_eq!(phone.page(), before);
    let refusal = phone.refusal().expect("the file is refused");
    assert!(refusal.starts_with("broken.toml") && refusal.contains(NOT_CHANGED));
    assert!(home.read(&progress).is_ok());
}

#[test]
fn a_file_that_is_not_text_is_refused_by_name() {
    let home = MemoryStorage::home();
    let mut phone = HeldPhone::on_the_sample(&home);
    phone.offer(&HeldPhone::picked("photo.heic", None));
    let refusal = phone.refusal().expect("the file is refused");
    assert!(refusal.starts_with("photo.heic") && refusal.contains(NOT_CHANGED));
}

#[test]
fn opening_the_shown_trip_again_keeps_its_progress_and_clears_an_old_refusal() {
    let home = MemoryStorage::home();
    let progress = HeldPhone::progress_kept_under(&home);
    let mut phone = HeldPhone::on_the_sample(&home);
    phone.offer(&HeldPhone::picked("broken.toml", Some("not a trip")));
    phone.offer(&HeldPhone::sample());
    assert!(phone.refusal().is_none());
    assert!(home.read(&progress).is_ok());
}

#[test]
fn the_chosen_trip_comes_back_at_the_next_launch_and_beats_the_documents_file() {
    let home = MemoryStorage::home();
    let mut phone = HeldPhone::on_the_sample(&home);
    phone.offer(&HeldPhone::later_trip());
    let chosen = phone.page();
    assert_eq!(HeldPhone::opened(&home).page(), chosen);
}

#[test]
fn the_built_in_trip_can_be_chosen_again() {
    let home = MemoryStorage::home();
    let mut phone = HeldPhone::on_the_sample(&home);
    let built_in = phone.page();
    phone.offer(&HeldPhone::later_trip());
    assert_ne!(phone.page(), built_in);
    phone.offer(&PickedFile::built_in());
    assert_eq!(phone.page(), built_in);
}

#[test]
fn a_chosen_trip_that_no_longer_loads_shows_why_and_offers_the_picker() {
    let home = MemoryStorage::home();
    let chosen = TripInput::chosen_under(&home);
    home.replace(chosen.as_ref(), &"not a trip").unwrap();
    let mut phone = HeldPhone::opened(&home);
    assert!(phone.page().contains(FAILED) && phone.page().contains(FILE_INPUT));
    phone.offer(&HeldPhone::sample());
    assert!(!phone.page().contains(FAILED));
}

#[test]
fn a_trip_the_phone_cannot_save_is_refused_and_the_trip_stays() {
    let home = MemoryStorage::home();
    let mut phone = HeldPhone::on_the_sample(&home);
    home.replace(&home.joined(["Library"]), &"in the way")
        .unwrap();
    let before = phone.page();
    phone.offer(&HeldPhone::later_trip());
    assert_eq!(phone.page(), before);
    let refusal = phone.refusal().expect("the file is refused");
    assert!(refusal.starts_with("later.toml") && refusal.contains(NOT_CHANGED));
}

#[test]
fn choosing_the_built_in_trip_follows_this_builds_copy_not_a_frozen_one() {
    let home = MemoryStorage::home();
    let mut phone = HeldPhone::on_the_sample(&home);
    phone.offer(&HeldPhone::later_trip());
    phone.offer(&PickedFile::built_in());
    let kept = TripInput::chosen_under(&home)
        .read::<String>(&home)
        .unwrap();
    assert_ne!(kept, TripInput::BUILT_IN);
    let opened = TripInput::read(None, Some(&home)).unwrap();
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

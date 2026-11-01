use super::shade::{Brightness, Contrast};
use super::sheet::Sheet;

const ROOT: &str = ":root";
const AFTER_SUNSET: &str = "html:has(.theme-dark)";
const FRAME: [&str; 4] = [".tabs", ".menu", ".wiki-bar", ".status-bar"];
const READING: Contrast = Contrast::new(7.0);
const SECONDARY: Contrast = Contrast::new(4.5);
const LIGHT_LEAST: Brightness = Brightness::new(0.7);
const DARK_MOST: Brightness = Brightness::new(0.1);

#[test]
fn reading_areas_are_light_and_the_frame_is_dark_and_both_read_in_greyscale() {
    let sheet = Sheet::main();
    let shade = |token| sheet.shade(ROOT, token);
    assert!(shade("--paper").brightness() >= LIGHT_LEAST);
    assert!(shade("--frame-paper").brightness() <= DARK_MOST);
    for (paper, ink, muted) in [
        ("--paper", "--ink", "--muted"),
        ("--frame-paper", "--frame-ink", "--frame-muted"),
    ] {
        assert!(shade(ink).contrast_with(shade(paper)) >= READING, "{ink}");
        assert!(
            shade(muted).contrast_with(shade(paper)) >= SECONDARY,
            "{muted}"
        );
    }
}

#[test]
fn the_phone_setting_never_darkens_a_reading_area() {
    let sheet = Sheet::main();
    assert_eq!(sheet.value(ROOT, "color-scheme"), Some("light"));
    assert!(!include_str!("../../../assets/main.css").contains("prefers-color-scheme"));
}

#[test]
fn every_part_of_the_frame_takes_the_frame_tokens() {
    let sheet = Sheet::main();
    let loose: Vec<&str> = FRAME
        .into_iter()
        .filter(|part| sheet.paper_of(part) != Some("var(--frame-paper)"))
        .collect();
    assert_eq!(loose, Vec::<&str>::new());
}

#[test]
fn the_dark_after_sunset_look_reads_in_greyscale_and_keeps_the_frame_apart() {
    let sheet = Sheet::main();
    let shade = |token| sheet.shade(AFTER_SUNSET, token);
    assert!(shade("--paper").brightness() <= DARK_MOST);
    assert!(shade("--ink").contrast_with(shade("--paper")) >= READING);
    assert!(shade("--muted").contrast_with(shade("--paper")) >= SECONDARY);
    assert_ne!(shade("--frame-paper"), shade("--paper"));
    assert!(
        sheet
            .shade(ROOT, "--frame-ink")
            .contrast_with(shade("--frame-paper"))
            >= READING
    );
}

#[test]
fn no_colour_is_written_outside_the_token_blocks() {
    assert_eq!(Sheet::main().colours_outside_tokens(), Vec::new());
}

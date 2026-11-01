use std::collections::BTreeSet;
use std::fs;

use sha1_smol::Sha1;

use crate::page_checks::RenderChecks;
use crate::ui_check::{Drawing, Harness, Stage};

pub struct Golden;

impl Golden {
    const KEPT: &str = include_str!("pages.sha1");
    const FILE: &str = "tests/api/page_checks/golden/pages.sha1";
    const BLESS: &str = "GOLDEN_BLESS";
    const DECIMALS_LEN: usize = 3;

    pub fn changed(stages: &[Stage]) -> Vec<String> {
        let rendered = RenderChecks::across(stages, Self::line);
        std::env::var_os(Self::BLESS)
            .map_or(Ok(()), |_| {
                fs::write(Self::FILE, rendered.join("\n") + "\n")
            })
            .expect("the goldens are writable");
        let now: BTreeSet<&str> = rendered.iter().map(String::as_str).collect();
        let kept: BTreeSet<&str> = Self::KEPT.lines().collect();
        now.symmetric_difference(&kept)
            .map(ToString::to_string)
            .collect()
    }

    fn line(stage: &Stage) -> Vec<String> {
        let drawn = stage.clone().drawn_with(Drawing::BuiltIn);
        let page = Self::hashed(&Harness::page(&drawn));
        vec![format!("{page} {stage}")]
    }

    fn hashed(page: &str) -> String {
        page.split_inclusive('.')
            .fold(Sha1::new(), |mut hash, part| {
                let digits = part.bytes().take_while(u8::is_ascii_digit).count();
                let kept = Self::DECIMALS_LEN.min(digits);
                hash.update(part.as_bytes().get(..kept).unwrap_or_default());
                hash.update(part.as_bytes().get(digits..).unwrap_or_default());
                hash
            })
            .hexdigest()
    }
}

#[test]
fn every_page_of_the_existing_trip_renders_as_it_did() {
    let (itinerary, wiki) = (Drawing::BuiltIn.itinerary(), Drawing::BuiltIn.wiki());
    let changed = Golden::changed(&Stage::all(&itinerary, &wiki));
    assert!(
        changed.is_empty(),
        "{} pages changed:\n{}",
        changed.len(),
        changed.join("\n")
    );
}

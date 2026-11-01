use std::collections::BTreeSet;
use std::fmt;

use crate::ui_check::{Drawing, Stage};

pub struct StyleGuard {
    hiding: Vec<String>,
}

impl StyleGuard {
    const HIDING: [&str; 7] = [
        "display:none",
        "visibility:hidden",
        "opacity:0",
        "width:0",
        "height:0",
        "max-width:0",
        "max-height:0",
    ];
    const STATE: [char; 2] = [':', '['];

    pub fn new(sheet: &str) -> StyleGuard {
        StyleGuard {
            hiding: sheet
                .split('}')
                .filter_map(|rule| rule.split_once('{'))
                .filter(|(_, body)| Self::hides(body))
                .flat_map(|(selectors, _)| selectors.split(',').map(str::trim))
                .filter(|selector| !selector.contains(Self::STATE))
                .map(str::to_owned)
                .collect(),
        }
    }

    pub fn hides(body: &str) -> bool {
        body.split(';')
            .map(|declaration| declaration.replace(char::is_whitespace, ""))
            .any(|declaration| Self::HIDING.contains(&declaration.as_str()))
    }

    pub fn classes(selector: &str) -> BTreeSet<&str> {
        selector
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .split('.')
            .skip(1)
            .collect()
    }

    pub fn hidden<'required>(&self, required: &[&'required str]) -> Vec<&'required str> {
        required
            .iter()
            .copied()
            .filter(|wanted| {
                self.hiding.iter().any(|rule| {
                    let hidden = Self::classes(rule);
                    !hidden.is_empty() && hidden.is_subset(&Self::classes(wanted))
                })
            })
            .collect()
    }
}

impl fmt::Display for StyleGuard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} hiding selectors", self.hiding.len())
    }
}

const SHEET: &str = include_str!("../../../assets/main.css");
const PLANTED: &str =
    ".x { color: red; }\n.wiki-kind { opacity: 0; }\n.menu:not(.open) .day-row { display: none; }";
const CAUGHT: [&str; 1] = ["article.wiki header .wiki-kind"];

#[test]
fn no_rule_hides_a_part_a_screen_requires() {
    let (itinerary, wiki) = (Drawing::BuiltIn.itinerary(), Drawing::BuiltIn.wiki());
    let stages = Stage::all(&itinerary, &wiki);
    let required: Vec<&str> = stages.iter().flat_map(Stage::required).collect();
    let hidden = StyleGuard::new(SHEET).hidden(&required);
    let planted = StyleGuard::new(PLANTED).hidden(&required);
    assert!(hidden.is_empty(), "hidden by the style sheet: {hidden:?}");
    assert_eq!(
        planted
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>(),
        CAUGHT.into()
    );
}

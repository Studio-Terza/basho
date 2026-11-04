mod boundary;
mod style_rule;
mod style_rules;

use std::fs;
use std::path::Path;

use basho_lints::Failure;
use basho_lints::Site;
use basho_lints::source_file::{RelativePath, SourceText};
pub use boundary::Quoting;
pub use style_rule::{Context, CssClass, CssValue, Declaration, Selector, StyleRule};
use style_rules::StyleRules;

pub struct StyleSheet {
    path: RelativePath,
    text: SourceText,
}

impl StyleSheet {
    const SHEET: &str = "crates/app/assets/main.css";

    pub fn read(root: &Path) -> Result<StyleSheet, Failure> {
        fs::read_to_string(root.join(Self::SHEET))
            .map(|text| StyleSheet {
                path: RelativePath::new(Self::SHEET.to_owned()),
                text: SourceText::new(text),
            })
            .map_err(|error| Failure::Io {
                site: Site::File(RelativePath::new(Self::SHEET.to_owned())),
                error,
            })
    }

    #[must_use]
    pub fn parse(path: RelativePath, text: SourceText) -> StyleSheet {
        StyleSheet { path, text }
    }

    #[must_use]
    pub fn path(&self) -> &RelativePath {
        &self.path
    }

    #[must_use]
    pub fn text(&self) -> &SourceText {
        &self.text
    }

    #[must_use]
    pub fn rules(&self) -> Vec<StyleRule> {
        let Ok(rules) = self.text.parse::<StyleRules>();
        rules.into_vec()
    }
}

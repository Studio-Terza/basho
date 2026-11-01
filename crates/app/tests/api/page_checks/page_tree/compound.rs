use std::convert::Infallible;
use std::str::FromStr;

use super::node::Node;

pub struct Compound {
    tag: Option<String>,
    classes: Vec<String>,
    absent_classes: Vec<String>,
    absent_descendants: Vec<String>,
}

impl Compound {
    const NOT: &str = ":not(";
    const HAS: &str = ":has(";

    pub fn fits(&self, node: &Node) -> bool {
        self.tag.as_deref().is_none_or(|tag| tag == node.tag())
            && self
                .classes
                .iter()
                .all(|class| node.classes().contains(class))
            && !self
                .absent_classes
                .iter()
                .any(|class| node.classes().contains(class))
    }

    pub fn absent_descendants(&self) -> &[String] {
        &self.absent_descendants
    }

    fn class_list(text: &str) -> Vec<String> {
        text.split(['.', ','])
            .map(str::trim)
            .filter(|class| !class.is_empty())
            .map(str::to_owned)
            .collect()
    }
}

impl FromStr for Compound {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Compound, Infallible> {
        let (base, negated) = text.split_once(Self::NOT).unwrap_or((text, ""));
        let negated = negated.trim_end_matches(')');
        let (tag, classes) = base.split_once('.').unwrap_or((base, ""));
        let (absent_classes, absent_descendants) = negated
            .strip_prefix(Self::HAS)
            .map_or((negated, ""), |descendants| ("", descendants));
        Ok(Compound {
            tag: Some(tag).filter(|tag| !tag.is_empty()).map(str::to_owned),
            classes: Self::class_list(classes),
            absent_classes: Self::class_list(absent_classes),
            absent_descendants: Self::class_list(absent_descendants),
        })
    }
}

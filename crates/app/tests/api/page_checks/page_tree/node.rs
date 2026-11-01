use std::convert::Infallible;
use std::str::FromStr;

use super::node_index::NodeIndex;
use super::text_len::TextLen;

pub struct Node {
    tag: String,
    classes: Vec<String>,
    label: Option<String>,
    len: TextLen,
    parent: Option<NodeIndex>,
}

impl Node {
    const CLASS: &str = "class=\"";
    const LABEL: &str = "aria-label=\"";
    const READING: &str = "rt";
    const BUTTON: &str = "button";

    pub fn under(self, parent: Option<NodeIndex>) -> Node {
        Node { parent, ..self }
    }

    pub fn parent(&self) -> Option<NodeIndex> {
        self.parent
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn classes(&self) -> &[String] {
        &self.classes
    }

    pub fn heard(&mut self, len: TextLen) {
        self.len = self.len + len;
    }

    pub fn spoken(&self) -> bool {
        self.len != TextLen::default() || self.label.is_some()
    }

    pub fn silence(&self) -> Option<&'static str> {
        (self.len == TextLen::default() && self.label.is_none())
            .then_some(self.tag.as_str())
            .and_then(|tag| match tag {
                Self::READING => Some("empty-reading: a reading has no text"),
                Self::BUTTON => Some("empty-button: a button has no label"),
                _ => None,
            })
    }

    fn quoted(tag: &str, attribute: &str) -> Option<String> {
        tag.split_once(attribute)
            .and_then(|(_, rest)| rest.split_once('"'))
            .map(|(value, _)| value.to_owned())
    }
}

impl FromStr for Node {
    type Err = Infallible;

    fn from_str(tag: &str) -> Result<Node, Infallible> {
        Ok(Node {
            tag: tag
                .split(|character: char| character.is_whitespace() || character == '/')
                .next()
                .unwrap_or_default()
                .to_owned(),
            classes: Self::quoted(tag, Self::CLASS)
                .map(|classes| classes.split_whitespace().map(str::to_owned).collect())
                .unwrap_or_default(),
            label: Self::quoted(tag, Self::LABEL),
            len: TextLen::default(),
            parent: None,
        })
    }
}

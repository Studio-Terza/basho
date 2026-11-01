mod compound;
mod node;
mod node_index;
mod selector;
mod text_len;

use std::convert::Infallible;
use std::str::FromStr;

use crate::ui_check::{Harness, Stage};
use compound::Compound;
use node::Node;
use node_index::NodeIndex;
use selector::Selector;
use text_len::TextLen;

#[derive(Default)]
pub struct PageTree {
    nodes: Vec<Node>,
    open: Vec<NodeIndex>,
    faults: Vec<String>,
}

impl PageTree {
    const JUNK: [&str; 3] = ["undefined", "NaN", "null"];
    const MARKUP_OPEN: char = '{';
    const MARKUP_SPLIT: char = '|';
    const MARKUP_CLOSE: char = '}';
    const VOID: [&str; 14] = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ];

    pub fn empty_element(tag: &str) -> bool {
        let name = tag
            .split(|character: char| character.is_whitespace() || character == '/')
            .next()
            .unwrap_or_default();
        tag.ends_with('/') || tag.starts_with('!') || Self::VOID.contains(&name)
    }

    pub fn stage_problems(stage: &Stage) -> Vec<String> {
        Harness::page(stage)
            .parse::<PageTree>()
            .unwrap_or_else(|never| match never {})
            .problems(&stage.required())
            .into_iter()
            .map(|problem| format!("{stage}: {problem}"))
            .collect()
    }

    pub fn problems(&self, required: &[&str]) -> Vec<String> {
        required
            .iter()
            .filter_map(|selector| self.unseen(selector))
            .chain(self.faults.iter().cloned())
            .collect()
    }

    pub fn unseen(&self, selector: &str) -> Option<String> {
        let found = self.matches(&selector.parse().unwrap_or_else(|never| match never {}));
        let missing = found
            .is_empty()
            .then(|| format!("missing: nothing matches {selector}"));
        let invisible = format!("invisible: {selector} has no text or content");
        (!found.iter().any(|index| self.shows(*index))).then(|| missing.unwrap_or(invisible))
    }

    pub fn matches(&self, selector: &Selector) -> Vec<NodeIndex> {
        selector
            .compounds()
            .split_last()
            .map(|(last, rest)| {
                self.indexed()
                    .filter(|index| {
                        last.fits(self.node(*index))
                            && self.lacks(*index, last.absent_descendants())
                            && self.descends(*index, rest)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn shows(&self, index: NodeIndex) -> bool {
        self.node(index).spoken()
            || self
                .indexed()
                .any(|other| self.node(other).parent() == Some(index))
    }

    pub fn descends(&self, index: NodeIndex, rest: &[Compound]) -> bool {
        self.ancestors(index).fold(rest.len(), |left, ancestor| {
            rest.get(left.wrapping_sub(1))
                .filter(|compound| compound.fits(self.node(ancestor)))
                .map_or(left, |_| left - 1)
        }) == 0
    }

    pub fn lacks(&self, index: NodeIndex, classes: &[String]) -> bool {
        classes.is_empty()
            || !self.indexed().any(|other| {
                self.node(other)
                    .classes()
                    .iter()
                    .any(|class| classes.contains(class))
                    && self.ancestors(other).any(|ancestor| ancestor == index)
            })
    }

    pub fn ancestors(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        std::iter::successors(self.node(index).parent(), |parent| {
            self.node(*parent).parent()
        })
    }

    pub fn indexed(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        (0..self.nodes.len()).map(NodeIndex::new)
    }

    pub fn node(&self, index: NodeIndex) -> &Node {
        &self.nodes[index.index()]
    }

    pub fn faults_in(text: &str) -> Vec<String> {
        let markup = text
            .split_once(Self::MARKUP_OPEN)
            .and_then(|(_, rest)| rest.split_once(Self::MARKUP_SPLIT))
            .and_then(|(_, rest)| rest.split_once(Self::MARKUP_CLOSE))
            .map(|_| format!("markup: unrendered ruby markup in \"{}\"", text.trim()));
        let junk = text
            .split(|character: char| !character.is_alphanumeric())
            .find(|word| Self::JUNK.contains(word))
            .map(|word| format!("junk: the page shows \"{word}\""));
        markup.into_iter().chain(junk).collect()
    }

    fn read(self, piece: &str) -> PageTree {
        let (tag, text) = piece.split_once('>').unwrap_or((piece, ""));
        let mut tree = if tag.starts_with('/') {
            self.closed()
        } else if Self::empty_element(tag) {
            self.opened(tag).closed()
        } else {
            self.opened(tag)
        }
        .heard(text.parse::<TextLen>().unwrap_or_default());
        tree.faults.extend(Self::faults_in(text));
        tree
    }

    fn opened(mut self, tag: &str) -> PageTree {
        let node = tag
            .parse::<Node>()
            .unwrap_or_else(|never| match never {})
            .under(self.open.last().copied());
        self.open.push(NodeIndex::new(self.nodes.len()));
        self.nodes.push(node);
        self
    }

    fn closed(mut self) -> PageTree {
        let silent = self
            .open
            .pop()
            .and_then(|index| self.nodes[index.index()].silence());
        self.faults.extend(silent.map(str::to_owned));
        self
    }

    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    fn heard(mut self, len: TextLen) -> PageTree {
        let PageTree { nodes, open, .. } = &mut self;
        open.iter()
            .for_each(|index| nodes[index.index()].heard(len));
        self
    }
}

impl FromStr for PageTree {
    type Err = Infallible;

    fn from_str(body: &str) -> Result<PageTree, Infallible> {
        Ok(body
            .split('<')
            .skip(1)
            .fold(PageTree::default(), PageTree::read))
    }
}

const BROKEN: &str = r#"<nav class="tabs"><button class="tab"></button></nav><p>{漢字|かんじ}</p><p>NaN km</p><ruby>字<rt></rt></ruby><div class="buffer short"><span class="spare inverted">5</span></div>"#;
const REQUIRED: [&str; 4] = [
    ".buffer.short .spare.inverted",
    ".buffer.short .spare:not(.inverted)",
    "nav.tabs .tab.active",
    "div.buffer:not(:has(.spare))",
];
const CAUGHT: [&str; 7] = [
    "markup",
    "junk",
    "empty-reading",
    "empty-button",
    ":not(.inverted)",
    ".tab.active",
    ":has(.spare)",
];

#[test]
fn rendered_html_is_checked_for_text_faults_and_required_parts_without_a_browser() {
    let problems = BROKEN
        .parse::<PageTree>()
        .unwrap()
        .problems(&REQUIRED)
        .join("\n");
    let missed: Vec<&str> = CAUGHT
        .into_iter()
        .filter(|kind| !problems.contains(kind))
        .collect();
    assert!(
        missed.is_empty() && !problems.contains(REQUIRED[0]),
        "not caught: {missed:?}\n{problems}"
    );
}

const SHAPES: &str = r#"<div class="a"><br><img src="x"><hr/><i class="dot"/></div><p class="tagline"></p><p class="b">x</p>"#;
const ASKED: [&str; 3] = [".a .b", "p.tagline", ".a .dot"];
const FOUND: [&str; 3] = [
    "missing: nothing matches .a .b",
    "invisible: p.tagline",
    "invisible: .a .dot",
];

#[test]
fn empty_tags_close_at_once_and_a_required_part_with_nothing_in_it_is_invisible() {
    let problems = SHAPES
        .parse::<PageTree>()
        .unwrap()
        .problems(&ASKED)
        .join("\n");
    let missed: Vec<&str> = FOUND
        .into_iter()
        .filter(|found| !problems.contains(found))
        .collect();
    assert!(missed.is_empty(), "not caught: {missed:?}\n{problems}");
}

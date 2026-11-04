use std::fmt;

mod valued;

use super::SelectorParts;
use super::name_length::NameLength;
use valued::Valued;

#[derive(
    newtype::Newtype,
    newtype::Arbitrary,
    newtype::Display,
    derive_more::Deref,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[deref(forward)]
pub struct PartName(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unread;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimplePart {
    Plain,
    Id(PartName),
    Attribute {
        name: PartName,
        valued: Valued,
    },
    PseudoClass {
        name: PartName,
        inner: Option<PartName>,
    },
    Sibling(char),
    AtRule,
    Nested,
}

impl SimplePart {
    const FIXED_ATTRIBUTES: [&str; 2] = ["role", "lang"];
    const STATIC_ATTRIBUTES: [&str; 3] = ["role", "lang", "open"];
    const KIND_STATES: [&str; 15] = [
        "root",
        "hover",
        "focus",
        "focus-visible",
        "focus-within",
        "active",
        "checked",
        "disabled",
        "enabled",
        "open",
        "visited",
        "link",
        "any-link",
        "placeholder-shown",
        "indeterminate",
    ];
    const OVER_KINDS: [&str; 4] = ["has", "not", "is", "where"];
    const ELEMENT_MARK: &str = "::";
    const SIBLINGS: [char; 2] = ['+', '~'];
    const NESTING: char = '&';

    pub fn refused(&self) -> Vec<SimplePart> {
        match self {
            SimplePart::Plain => Vec::new(),
            SimplePart::Id(_)
            | SimplePart::Sibling(_)
            | SimplePart::AtRule
            | SimplePart::Nested => {
                vec![self.clone()]
            }
            SimplePart::Attribute { name, valued } => (!Self::STATIC_ATTRIBUTES.contains(&&**name)
                || (*valued == Valued::Matched && !Self::FIXED_ATTRIBUTES.contains(&&**name)))
            .then(|| self.clone())
            .into_iter()
            .collect(),
            SimplePart::PseudoClass { name, inner } => inner
                .as_ref()
                .filter(|_| Self::OVER_KINDS.contains(&&**name))
                .map(|inner| {
                    let Ok(parts) = inner.parse::<SelectorParts>();
                    parts.refused()
                })
                .or_else(|| Self::KIND_STATES.contains(&&**name).then(Vec::new))
                .unwrap_or_else(|| vec![self.clone()]),
        }
    }
}

impl TryFrom<&str> for SimplePart {
    type Error = Unread;

    fn try_from(token: &str) -> Result<SimplePart, Unread> {
        let name = |from: usize| {
            PartName::new(
                token[from..from + usize::from(NameLength::from(&token[from..]))].to_owned(),
            )
        };
        if let Some(rest) = token.strip_prefix('[') {
            let inner = rest.strip_suffix(']').unwrap_or(rest).trim();
            let named = inner.find(Valued::NAME_END).unwrap_or(inner.len());
            Ok(SimplePart::Attribute {
                name: PartName::new(inner[..named].to_owned()),
                valued: Valued::from(inner),
            })
        } else if token.starts_with(Self::ELEMENT_MARK) {
            Ok(SimplePart::Plain)
        } else if token.starts_with(':') {
            let called = name(1);
            let group = &token[1 + called.len()..];
            Ok(SimplePart::PseudoClass {
                inner: group
                    .strip_prefix('(')
                    .and_then(|group| group.strip_suffix(')'))
                    .map(|inner| PartName::new(inner.to_owned())),
                name: called,
            })
        } else if token.starts_with('#') {
            Ok(SimplePart::Id(name(1)))
        } else if let Some(mark) = token
            .chars()
            .next()
            .filter(|mark| Self::SIBLINGS.contains(mark))
        {
            Ok(SimplePart::Sibling(mark))
        } else if token.starts_with(Self::NESTING) {
            Ok(SimplePart::Nested)
        } else {
            (token.starts_with('.')
                || token.starts_with('*')
                || usize::from(NameLength::from(token)) > 0)
                .then_some(SimplePart::Plain)
                .ok_or(Unread)
        }
    }
}

impl fmt::Display for SimplePart {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimplePart::Plain => formatter.write_str("a kind or shared state"),
            SimplePart::Id(name) => write!(formatter, "#{name} picks one element by its id"),
            SimplePart::Attribute { name, .. } => write!(
                formatter,
                "[{name}=…] selects an attribute that markup may set per instance; only role, lang and open are static"
            ),
            SimplePart::PseudoClass { name, .. } => write!(
                formatter,
                ":{name} can pick out one instance among its siblings; style by kind and state instead"
            ),
            SimplePart::Sibling(mark) => write!(
                formatter,
                "the {mark} combinator picks an element by its position among siblings"
            ),
            SimplePart::AtRule => formatter.write_str(
                "an at-rule the lint does not read; use @media, @supports, @layer, @container or @starting-style",
            ),
            SimplePart::Nested => formatter.write_str(
                "nested rules are not read by the lint; write each rule at the top level",
            ),
        }
    }
}

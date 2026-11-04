mod words;

use std::fmt;

use serde::de::value::MapAccessDeserializer;
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::content::{NameKind, TypedPart};
use crate::ruby_text::RubyText;

use words::Words;

#[derive(Clone, Debug)]
pub enum Part {
    Words(Words),
    Typed(TypedPart),
}

impl Part {
    #[must_use]
    pub fn shown(&self) -> RubyText {
        match self {
            Part::Words(words) => AsRef::<RubyText>::as_ref(words).clone(),
            Part::Typed(typed) => typed.shown(),
        }
    }

    #[must_use]
    pub fn named(&self, kind: NameKind) -> Option<&RubyText> {
        match self {
            Part::Words(_) => None,
            Part::Typed(typed) => typed.named(kind),
        }
    }

    #[must_use]
    pub fn words(&self) -> Option<&Words> {
        match self {
            Part::Words(words) => Some(words),
            Part::Typed(_) => None,
        }
    }
}

struct PartVisitor;

impl<'de> Visitor<'de> for PartVisitor {
    type Value = Part;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("connecting words, or a typed part such as { plan = \"13:15\" }")
    }

    fn visit_str<Failure: Error>(self, text: &str) -> Result<Part, Failure> {
        Words::try_from(text.to_owned())
            .map(Part::Words)
            .map_err(Failure::custom)
    }

    fn visit_map<Entries: MapAccess<'de>>(self, entries: Entries) -> Result<Part, Entries::Error> {
        TypedPart::deserialize(MapAccessDeserializer::new(entries)).map(Part::Typed)
    }
}

impl<'de> Deserialize<'de> for Part {
    fn deserialize<Input: Deserializer<'de>>(input: Input) -> Result<Part, Input::Error> {
        input.deserialize_any(PartVisitor)
    }
}

use crate::ruby_text::RubyText;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameKind {
    Entry,
    Spot,
}

impl NameKind {
    pub(super) fn kept(self, wanted: NameKind, name: &RubyText) -> Option<&RubyText> {
        (self == wanted).then_some(name)
    }
}

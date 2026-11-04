use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::ruby_text::RubyText;
use newtype::Newtype;

#[derive(Newtype)]
pub struct RubyView<'text> {
    text: &'text RubyText,
}

impl IntoDynNode for RubyView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        rsx! {
            {self.text.segments().iter().map(|segment| segment.reading().map_or_else(
                || rsx! { "{segment.text()}" },
                |reading| rsx! {
                    ruby { "{segment.text()}" rt { "{reading}" } }
                },
            ))}
        }
        .into_dyn_node()
    }
}

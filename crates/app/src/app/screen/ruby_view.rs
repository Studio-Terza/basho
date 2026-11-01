use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::place::RubyText;
use newtype::Newtype;

#[derive(Newtype)]
pub struct RubyView<'text> {
    text: &'text RubyText,
}

impl IntoDynNode for RubyView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        rsx! {
            {self.text.segments().iter().map(|segment| match segment.reading() {
                Some(reading) => rsx! {
                    ruby { "{segment.text()}" rt { "{reading}" } }
                },
                None => rsx! { "{segment.text()}" },
            })}
        }
        .into_dyn_node()
    }
}

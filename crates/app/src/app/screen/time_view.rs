use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::times::{ListedTime, ShownTime};
use guide::words::Spelling;

pub struct TimeView {
    time: ShownTime,
    word: Option<Spelling>,
}

impl TimeView {
    pub fn new(time: impl Into<ShownTime>) -> Self {
        TimeView {
            time: time.into(),
            word: None,
        }
    }

    pub fn worded(time: ListedTime, word: Spelling) -> Self {
        TimeView {
            time: ShownTime::from(time),
            word: time.missing(word),
        }
    }
}

impl IntoDynNode for TimeView {
    fn into_dyn_node(self) -> DynamicNode {
        let time = self.time;
        rsx! {
            {self.word.map(|word| rsx! { "{word} " })}
            span { class: time.class(), "{time}" }
        }
        .into_dyn_node()
    }
}

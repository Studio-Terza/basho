use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::Card;
use guide::Language;
use guide::card::CardFill;
use guide::card::CardLine;
use guide::classed::Classed;
use guide::english_name::EnglishName;
use guide::itinerary::{Caption, Day};

use crate::app::screen::ruby_view::RubyView;

pub struct AskCard<'card> {
    card: &'card Card,
    day: &'card Day,
    language: Language,
}

impl<'card> AskCard<'card> {
    pub fn new(card: &'card Card, day: &'card Day, language: Language) -> Self {
        AskCard {
            card,
            day,
            language,
        }
    }
}

impl IntoDynNode for AskCard<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let language = self.language;
        let inn = self
            .card
            .fill()
            .filter(|fill| *fill == CardFill::TonightsInn)
            .and(self.day.night());
        rsx! {
            details { class: "ask",
                summary { class: "ask-why", "{self.card.line_text(CardLine::Purpose)}" }
                p { class: "japanese", lang: language.class(), {RubyView::new(self.card.line_text(CardLine::Japanese))} }
                {inn.map(|night| rsx! {
                    p { class: "japanese", lang: language.class(), {RubyView::new(&night[Caption::Name])} }
                })}
                p { class: "english", "{self.card.line_text(CardLine::English)}" }
                {inn.map(|night| rsx! { p { class: "english", "{AsRef::<EnglishName>::as_ref(night)}" } })}
            }
        }
        .into_dyn_node()
    }
}

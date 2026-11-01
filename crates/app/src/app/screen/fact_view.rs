use dioxus::prelude::*;
use guide::Language;
use guide::classed::Classed;
use guide::place::Fact;

use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::unverified_mark::UnverifiedMark;
use crate::app::screen::wording::{Rewording, Wording};
use crate::app::session::Session;

pub struct FactView<'fact> {
    fact: &'fact Fact,
    session: &'fact Session,
}

impl<'fact> FactView<'fact> {
    pub fn new(fact: &'fact Fact, session: &'fact Session) -> Self {
        FactView { fact, session }
    }

    pub fn render_sourced(&self) -> Element {
        let language = self.session.language();
        rsx! {
            div { class: "fact",
                p { lang: language.class(), {LinkedView::reworded(self.session, self.fact.text(), &Wording::new(self.fact.text()).reworded(Rewording::ForWalker).ruby()).into_dyn_node()} }
                {self.status_line(language)}
            }
        }
    }

    pub fn render_marked(self) -> Element {
        rsx! {
            {LinkedView::reworded(self.session, self.fact.text(), &Wording::new(self.fact.text()).reworded(Rewording::ForWalker).ruby()).into_dyn_node()}
            {UnverifiedMark::new(self.fact.evidence())}
        }
    }

    fn status_line(&self, language: Language) -> Element {
        let kind = self.fact.evidence().kind();
        self.fact.evidence().backing().map_or_else(
            || {
                rsx! {
                    p { class: "status unverified", lang: language.class(),
                        "{kind.word()}: "
                        {self.fact.evidence().reason().map(|reason| RubyView::new(&Wording::new(reason).reworded(Rewording::ForWalker).ruby()).into_dyn_node())}
                    }
                }
            },
            |source| {
                rsx! {
                    p { class: "status verified",
                        "{kind.word()} {source.checked()} · "
                        span { class: "source", "{source.url().host()}" }
                    }
                }
            },
        )
    }
}

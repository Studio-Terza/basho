use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::place::{Excerpt, ExcerptCredit, ExcerptText};

use crate::app::screen::linked_view::LinkedView;
use crate::app::session::Session;

pub struct ExcerptView<'excerpt> {
    excerpt: &'excerpt Excerpt,
    session: &'excerpt Session,
}

impl<'excerpt> ExcerptView<'excerpt> {
    pub fn new(excerpt: &'excerpt Excerpt, session: &'excerpt Session) -> Self {
        ExcerptView { excerpt, session }
    }
}

impl IntoDynNode for ExcerptView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let excerpt = self.excerpt;
        let language = self.session.language();
        rsx! {
            figure { class: "excerpt",
                figcaption { "{excerpt.year()} · {excerpt.credit(ExcerptCredit::Author)}, {excerpt.credit(ExcerptCredit::Work)}" }
                blockquote { class: "original", lang: language.class(), "{excerpt.text(ExcerptText::Original)}" }
                blockquote { class: "translation",
                    span { class: "label", "{excerpt.translator().label()}" }
                    {LinkedView::new(self.session, excerpt.text(ExcerptText::Translation))}
                }
                {excerpt.provenance().map(|provenance| rsx! { p { class: "provenance", "{provenance}" } })}
                p { class: "status verified",
                    "Fetched {excerpt.source().checked()} · "
                    span { class: "source", "{excerpt.source().url().host()}" }
                }
            }
        }
        .into_dyn_node()
    }
}

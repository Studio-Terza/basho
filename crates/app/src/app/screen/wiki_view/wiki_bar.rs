use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use wiki::EntryKey;
use wiki::entry::Entry;

use crate::app::history::ScrollTop;
use crate::app::screen::origin_name::OriginName;
use crate::app::screen::wiki_view::section_anchor::SectionAnchor;
use crate::app::session::{Session, SessionEvent};
use crate::app::step::Toward;
use crate::app::visit::Screen;

pub struct WikiBar<'view> {
    session: &'view Session,
    entry: &'view Entry,
}

impl<'view> WikiBar<'view> {
    const JUMP: &str = "const section = document.getElementById(await dioxus.recv()); const bar = section.closest('.wiki').querySelector('.wiki-bar'); window.scrollTo(0, section.getBoundingClientRect().top + window.scrollY - bar.offsetHeight);";

    pub fn new(session: &'view Session, entry: &'view Entry) -> Self {
        WikiBar { session, entry }
    }
}

impl IntoDynNode for WikiBar<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let language = self.session.language();
        let origin = self.session.history().origin().map(|mark| mark.screen());
        let back = self.session.clone();
        let opener = self.session.clone();
        let index = self.session.wiki().find(&EntryKey::Index);
        let shown = self.session.reader();
        let sections = self.entry.sections();
        rsx! {
            div { class: "wiki-bar",
                div { class: "wiki-top",
                    button { class: "wiki-back", lang: language.class(),
                        onclick: move |event| {
                            let top = ScrollTop::from(&event);
                            origin
                                .map_or(
                                    SessionEvent::OpenTab { screen: Screen::Today, top },
                                    |_| SessionEvent::TurnHistory { toward: Toward::Previous, top },
                                )
                                .applied(&back);
                        },
                        "‹ " {OriginName::new(self.session, origin.unwrap_or(Screen::Today))}
                    }
                    {index.filter(|index| Some(*index) != shown).map(|entry| rsx! {
                        button { class: "wiki-index",
                            onclick: move |event| SessionEvent::Follow { entry, top: ScrollTop::from(&event) }.applied(&opener),
                            "Index"
                        }
                    })}
                }
                {(sections.len() > 1).then(|| rsx! {
                    nav { class: "section-bar", "aria-label": "Sections",
                        {sections.iter().enumerate().map(|(position, section)| rsx! {
                            button { class: "section-jump",
                                onclick: move |_| {
                                    let jumper = document::eval(Self::JUMP);
                                    jumper.send(SectionAnchor::new(position).to_string()).unwrap_or_default();
                                },
                                "{section.heading()}"
                            }
                        })}
                    }
                })}
            }
        }
        .into_dyn_node()
    }
}

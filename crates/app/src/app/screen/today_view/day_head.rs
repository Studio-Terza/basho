use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::ArrivalMark;
use guide::FactSource;
use guide::classed::Classed;
use guide::content::Content;
use guide::end::End;
use guide::guidance::Guidance;
use guide::itinerary::Caption;
use guide::place::NoteKind;
use guide::times::FactTime;
use wiki::EntryKey;

use crate::app::screen::fact_view::FactView;
use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::time_view::TimeView;
use crate::app::screen::unverified_mark::UnverifiedMark;
use crate::app::session::Session;

pub struct DayHead<'view> {
    session: &'view Session,
    guidance: &'view Guidance<'view>,
}

impl<'view> DayHead<'view> {
    pub fn new(session: &'view Session, guidance: &'view Guidance<'view>) -> Self {
        DayHead { session, guidance }
    }
}

impl IntoDynNode for DayHead<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let language = self.session.language();
        let day = self.guidance.day();
        let now = self.guidance.now();
        let inn = self.session.wiki().find(&EntryKey::Inn(day.number()));
        let arrival = self.guidance.arrival().filter(|arrival| {
            arrival.time(ArrivalMark::AfterSunset).is_some()
                || arrival.time(ArrivalMark::AfterCheckIn).is_some()
        });
        rsx! {
            header { class: "day-head",
                p { class: "why", lang: language.class(), {LinkedView::new(self.session, AsRef::<Content>::as_ref(day))} }
                p { class: "leave", lang: language.class(),
                    {day.legs().first().map(|first| first.listed(End::Start)).map(|leaves| rsx! { "Leave at " strong { {TimeView::new(leaves)} } ". " })}
                    {day.night().map(|night| rsx! {
                        "Inn: " strong { {inn.map_or_else(
                            || rsx! { {RubyView::new(&night[Caption::Name])} },
                            |inn| LinkedView::entry(self.session, inn, &night[Caption::Name]),
                        )} {UnverifiedMark::new(night.as_ref())} }
                        " · " {TimeView::worded(night.listed_check_in(), FactSource::CheckIn.word().lowered())}
                        "."
                    })}
                }
                {day.legs().first().into_iter().flat_map(guide::leg::Leg::notes).filter(|fact| fact.kind() == NoteKind::Warning).map(|fact| rsx! {
                    p { class: "must", lang: language.class(), "‼ " {FactView::new(fact, self.session).render_marked()} }
                })}
                {arrival.map(|arrival| rsx! {
                    p { class: "must",
                        {arrival.arrives(now).map_or_else(
                            || rsx! { "‼ At this pace no ride takes you on to the inn today, after " },
                            |arrives| rsx! { "‼ At this pace you reach the inn at " {TimeView::new(arrives)} ", after " },
                        )}
                        {arrival.time(ArrivalMark::AfterSunset).map(|time| TimeView::new(FactTime::at_sunset(time)))}
                        {arrival.time(ArrivalMark::AfterSunset).and(arrival.time(ArrivalMark::AfterCheckIn)).map(|_| rsx! { " and " })}
                        {arrival.time(ArrivalMark::AfterCheckIn).and(day.night()).map(|night| TimeView::worded(night.listed_check_in(), FactSource::CheckIn.word().lowered()))}
                        "."
                    }
                })}
            }
        }
        .into_dyn_node()
    }
}

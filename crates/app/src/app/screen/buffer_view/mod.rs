mod buffer_shape;
mod spare_look;

use dioxus::prelude::*;
use guide::BackupMark;
use guide::FactSource;
use guide::Language;
use guide::classed::Classed;
use guide::timeline::{Buffer, BufferState, DeadlineKind};

use crate::app::screen::look::Look;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::time_view::TimeView;
use crate::app::screen::unverified_mark::UnverifiedMark;
pub use buffer_shape::BufferShape;
use spare_look::SpareLook;

pub struct BufferView<'view> {
    buffer: &'view Buffer<'view>,
    language: Language,
}

impl<'view> BufferView<'view> {
    pub fn new(buffer: &'view Buffer<'view>, language: Language) -> Self {
        BufferView { buffer, language }
    }

    pub fn render(self, shape: BufferShape) -> Element {
        let language = self.language;
        let buffer = self.buffer;
        let state = buffer.state();
        match shape {
            BufferShape::Full => {
                let advice = (state == BufferState::Short
                    && matches!(
                        buffer.deadline().kind(),
                        DeadlineKind::Ride | DeadlineKind::Tour
                    )
                    && buffer.reaches_before_event())
                .then_some("Walk on now. You can still make it if you do not stop.");
                let backup = buffer.backup();
                let kind = Look::deadline(buffer.deadline().kind());
                let tone = Look::buffer(state);
                let spare = SpareLook::for_state(state);
                rsx! {
                    section { class: "buffer {kind.class()} {tone.class()}", role: "group", "aria-label": "Time to spare",
                        p { class: "buffer-line", lang: language.class(),
                            span { class: "deadline", {self.deadline()} }
                            " · "
                            span { class: spare.class(),
                                {match state {
                                    BufferState::Comfortable => rsx! { "{buffer.gap()}" },
                                    BufferState::Tight => rsx! { "{buffer.gap()} · {buffer.deadline().haste()}" },
                                    BufferState::Short => rsx! { "{buffer.gap()}" },
                                    BufferState::Waiting => rsx! { "WAITING · leaves " {TimeView::new(buffer.deadline().listed().end)} " · {buffer.spare()}" },
                                }}
                            }
                        }
                        {advice.map(|words| rsx! { p { class: "buffer-advice", "{words}" } })}
                        {backup.map(|backup| rsx! {
                            p { class: "if-missed",
                                {match (backup.leaves().map(TimeView::new), backup.arrives().map(TimeView::new), backup.time(BackupMark::CheckIn)) {
                                    (None, _, _) => rsx! { "If missed: no later ride is listed." },
                                    (Some(leaves), None, _) => rsx! { "If missed: " {leaves} ", then no later ride is listed." },
                                    (Some(leaves), Some(arrives), Some(_)) => rsx! { "If missed: " {leaves} " → inn " {arrives} },
                                    (Some(leaves), Some(arrives), None) => rsx! { "If missed: " {leaves} " → arrive " {arrives} },
                                }}
                                {backup.after_check_in().map(|late| rsx! { " ({late} after check-in)" })}
                            }
                        })}
                    }
                }
            }
            BufferShape::Brief => rsx! {
                p { class: "goal-spare", lang: language.class(),
                    span { class: "deadline", {self.deadline()} }
                    " · "
                    {if state == BufferState::Short {
                        rsx! { span { class: "spare inverted", "{buffer.gap()}" } }
                    } else if state == BufferState::Waiting {
                        rsx! { span { class: "spare", "WAITING · leaves " {TimeView::new(buffer.deadline().listed().end)} } }
                    } else {
                        rsx! { span { class: "spare", "{buffer.gap()}" } }
                    }}
                }
            },
            BufferShape::Live => rsx! {
                span { class: "deadline", {self.deadline()} }
                " · "
                {match state {
                    BufferState::Comfortable => rsx! { span { class: "spare", "{buffer.gap()}" } },
                    BufferState::Tight | BufferState::Short => rsx! { span { class: "spare inverted", "{buffer.gap()}" } },
                    BufferState::Waiting => rsx! { span { class: "spare", "WAITING · leaves " {TimeView::new(buffer.deadline().listed().end)} } },
                }}
            },
        }
    }

    fn deadline(&self) -> Element {
        let deadline = self.buffer.deadline();
        let place = rsx! { {RubyView::new(deadline.place())} {deadline.place_evidence().map(UnverifiedMark::new)} };
        let event = deadline.listed().end;
        let target = TimeView::new(deadline.listed().start);
        match deadline.kind() {
            DeadlineKind::Ride => {
                let mode = deadline.mode().label().lowered();
                rsx! { {TimeView::worded(event, mode)} " · be at " {place} " by " {target} }
            }
            DeadlineKind::Tour => {
                rsx! { {TimeView::worded(event, FactSource::TourTimes.word().lowered())} " · be at " {place} " by " {target} }
            }
            DeadlineKind::Closes => {
                rsx! { {place} " " {TimeView::worded(event, FactSource::Closes.word().lowered())} }
            }
            DeadlineKind::Entry => {
                rsx! { {place} " " {TimeView::worded(event, FactSource::Entry.word().lowered())} }
            }
            DeadlineKind::LastOrder => {
                rsx! { {place} " " {TimeView::worded(event, FactSource::LastOrder.word().lowered())} }
            }
            DeadlineKind::CheckIn => {
                rsx! { {place} " " {TimeView::worded(event, FactSource::CheckIn.word().lowered())} }
            }
        }
    }
}

mod buffer_shape;
mod spare_look;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::BackupMark;
use guide::BufferState;
use guide::DeadlineKind;
use guide::FactSource;
use guide::Language;
use guide::classed::Classed;
use guide::guidance::timeline::Buffer;

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
        match shape {
            BufferShape::Full => self.full(),
            BufferShape::Brief => rsx! {
                p { class: "goal-spare", lang: self.language.class(),
                    span { class: "deadline", {self.deadline()} }
                    " · "
                    {if self.buffer.state() == BufferState::Short {
                        rsx! { span { class: "spare inverted", "{self.buffer.gap()}" } }
                    } else if self.buffer.state() == BufferState::Waiting {
                        rsx! { span { class: "spare", "WAITING · leaves " {TimeView::new(self.buffer.deadline().listed().end)} } }
                    } else {
                        rsx! { span { class: "spare", "{self.buffer.gap()}" } }
                    }}
                }
            },
            BufferShape::Live => rsx! {
                span { class: "deadline", {self.deadline()} }
                " · "
                {match self.buffer.state() {
                    BufferState::Comfortable => rsx! { span { class: "spare", "{self.buffer.gap()}" } },
                    BufferState::Tight | BufferState::Short => rsx! { span { class: "spare inverted", "{self.buffer.gap()}" } },
                    BufferState::Waiting => rsx! { span { class: "spare", "WAITING · leaves " {TimeView::new(self.buffer.deadline().listed().end)} } },
                }}
            },
        }
    }

    fn full(self) -> Element {
        let language = self.language;
        let buffer = self.buffer;
        let state = buffer.state();
        let advice = Some(state)
            .filter(|state| *state == BufferState::Short)
            .filter(|_| {
                matches!(
                    buffer.deadline().kind(),
                    DeadlineKind::Ride | DeadlineKind::Tour
                )
            })
            .filter(|_| buffer.reaches_before_event())
            .map(|_| "Walk on now. You can still make it if you do not stop.");
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

    fn place(&self) -> DynamicNode {
        let deadline = self.buffer.deadline();
        rsx! { {RubyView::new(deadline.place())} {deadline.place_evidence().map(UnverifiedMark::new)} }
            .into_dyn_node()
    }

    fn deadline(&self) -> Element {
        match self.buffer.deadline().kind() {
            DeadlineKind::Ride => {
                rsx! { {TimeView::worded(self.buffer.deadline().listed().end, self.buffer.deadline().mode().label().lowered())} " · be at " {self.place()} " by " {TimeView::new(self.buffer.deadline().listed().start)} }
            }
            DeadlineKind::Tour => {
                rsx! { {TimeView::worded(self.buffer.deadline().listed().end, FactSource::TourTimes.word().lowered())} " · be at " {self.place()} " by " {TimeView::new(self.buffer.deadline().listed().start)} }
            }
            DeadlineKind::Closes => {
                rsx! { {self.place()} " " {TimeView::worded(self.buffer.deadline().listed().end, FactSource::Closes.word().lowered())} }
            }
            DeadlineKind::Entry => {
                rsx! { {self.place()} " " {TimeView::worded(self.buffer.deadline().listed().end, FactSource::Entry.word().lowered())} }
            }
            DeadlineKind::LastOrder => {
                rsx! { {self.place()} " " {TimeView::worded(self.buffer.deadline().listed().end, FactSource::LastOrder.word().lowered())} }
            }
            DeadlineKind::CheckIn => {
                rsx! { {self.place()} " " {TimeView::worded(self.buffer.deadline().listed().end, FactSource::CheckIn.word().lowered())} }
            }
        }
    }
}

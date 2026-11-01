mod history_event;
mod menu_event;
mod pin_event;
mod progress_event;
mod report_event;
mod scroll_event;
mod touch_event;
use dioxus::prelude::*;
use guide::map_frame::{Pixels, PlanePoint};

use crate::app::history::ScrollTop;
use crate::app::screen::Screen;
use crate::app::session::Session;
use crate::app::session::gesture::Gesture;
use crate::app::step::{Spot, Toward};
use history_event::HistoryEvent;
use menu_event::MenuEvent;
use pin_event::PinEvent;
use progress_event::ProgressEvent;
use report_event::ReportEvent;
use scroll_event::ScrollEvent;
use touch_event::TouchEvent;
use wiki::EntryId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SessionEvent {
    TrackWalk,
    ConfirmProgress,
    NoteDecision,
    CaptureReport,
    ExportReports,
    ShowReports,
    PlaceHere(Spot),
    MarkDone(Spot),
    ShowCurrentStep,
    BackToNow,
    ShowFocusInToday,
    OpenMenu,
    CloseMenu,
    PageTouchBegan(PlanePoint),
    MenuTouchBegan(PlanePoint),
    PairTouched { at: PlanePoint, spread: Pixels },
    PairSpread(Pixels),
    PairHeld(Gesture),
    TouchMoved(PlanePoint),
    TouchEnded(PlanePoint),
    TouchCancelled,
    Remember(ScrollTop),
    Follow { entry: EntryId, top: ScrollTop },
    OpenTab { screen: Screen, top: ScrollTop },
    TurnHistory { toward: Toward, top: ScrollTop },
    NoteTouchTop(ScrollTop),
    SwitchMapMode,
    SwitchDark,
}

impl SessionEvent {
    pub fn applied(self, session: &Session) {
        match self {
            SessionEvent::TrackWalk => ProgressEvent::TrackWalk.progressed(session),
            SessionEvent::ConfirmProgress => ProgressEvent::Confirm.progressed(session),
            SessionEvent::NoteDecision => ProgressEvent::NoteDecision.progressed(session),
            SessionEvent::CaptureReport => ReportEvent::Capture.reported(session),
            SessionEvent::ExportReports => ReportEvent::Export.reported(session),
            SessionEvent::ShowReports => ReportEvent::Show.reported(session),
            SessionEvent::PlaceHere(spot) => PinEvent::Here(spot).pinned(session),
            SessionEvent::MarkDone(spot) => PinEvent::Done(spot).pinned(session),
            SessionEvent::ShowCurrentStep => ScrollEvent::ShowCurrentStep.scrolled(session),
            SessionEvent::BackToNow => ScrollEvent::BackToNow.scrolled(session),
            SessionEvent::ShowFocusInToday => ScrollEvent::ShowFocusInToday.scrolled(session),
            SessionEvent::OpenMenu => MenuEvent::Open.moved_menu(session),
            SessionEvent::CloseMenu => MenuEvent::Close.moved_menu(session),
            SessionEvent::PageTouchBegan(at) => TouchEvent::PageBegan(at).touched(session),
            SessionEvent::MenuTouchBegan(at) => TouchEvent::MenuBegan(at).touched(session),
            SessionEvent::PairTouched { at, spread } => {
                TouchEvent::PairBegan { at, spread }.touched(session);
            }
            SessionEvent::PairSpread(now) => TouchEvent::PairSpread(now).touched(session),
            SessionEvent::PairHeld(paired) => TouchEvent::PairHeld(paired).touched(session),
            SessionEvent::TouchMoved(at) => TouchEvent::Moved(at).touched(session),
            SessionEvent::TouchEnded(at) => TouchEvent::Ended(at).touched(session),
            SessionEvent::TouchCancelled => TouchEvent::Cancelled.touched(session),
            SessionEvent::Remember(top) => HistoryEvent::Remember(top).moved(session),
            SessionEvent::Follow { entry, top } => {
                HistoryEvent::Follow { entry, top }.moved(session);
            }
            SessionEvent::OpenTab { screen, top } => {
                HistoryEvent::OpenTab { screen, top }.moved(session);
            }
            SessionEvent::TurnHistory { toward, top } => {
                HistoryEvent::Turn { toward, top }.moved(session);
            }
            SessionEvent::NoteTouchTop(top) => {
                let mut touched = session.touch_top;
                touched.set(top);
            }
            SessionEvent::SwitchMapMode => {
                let mut mode = session.map_mode;
                let switched = mode.peek().switched();
                mode.set(switched);
            }
            SessionEvent::SwitchDark => {
                let mut dark = session.dark;
                let switched = dark.peek().switched();
                switched.store();
                dark.set(switched);
            }
        }
    }
}

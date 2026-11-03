use std::cell::RefCell;
use std::path::PathBuf;

use dioxus::prelude::*;
use guide::clock::{ClockTime, Minutes, TripTime};
use guide::guidance::{Guidance, LegProgress};
use guide::map_frame::{Pixels, PlanePoint};
use layout::Touch;
use report::{Fact, Report, ReportBook, ReportStore, ReportText};
use serde::Deserialize;
use storage_fake::MemoryStorage;
use storage_ports::HomeFolder;

use basho::app::session::gesture::Gesture;
use basho::app::session::tracker_log::{DecisionLog, TrackerDecision};
use basho::app::session::{Session, SessionEvent};
use basho::app::step::{Spot, StepNumber};
use basho::app::visit::Screen;

use crate::ui_check::{Drawing, Harness, Stage};

const WALKED: Minutes = Minutes::new(1);
const EARLIER: StepNumber = StepNumber::from_position(0);
const LATER: StepNumber = StepNumber::from_position(1);

thread_local! {
    static AFTER: RefCell<Option<After>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug, PartialEq)]
struct After {
    screen: Screen,
    book: ReportBook,
    gesture: Gesture,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Act {
    CaptureAfterTwoDecisions,
    CaptureWhileBrowsing,
    HoldTwoFingers,
    MoveTwoFingers,
    SpreadOneFinger,
    LiftTwoFingers,
    HoldAgainAfterCapture,
    CaptureAndExport,
    CaptureAndNote,
}

#[derive(Props, Clone, PartialEq)]
struct Scene {
    stage: Stage,
    act: Act,
}

impl Scene {
    fn after(stage: Stage, act: Act) -> After {
        let mut dom = VirtualDom::new_with_props(Self::render, Scene { stage, act });
        dom.rebuild_in_place();
        AFTER.take().expect("the scene ran")
    }

    fn render(scene: Scene) -> Element {
        let Scene { stage, act } = scene;
        let session = Session::staged(
            stage.drawing().itinerary(),
            stage.drawing().places(),
            stage.drawing().overview(),
            &stage,
        );
        use_hook(|| {
            act.done_on(&session);
            AFTER.set(Some(After {
                screen: session.screen(),
                book: session.report_book(),
                gesture: session.gesture(),
            }));
        });
        rsx! {}
    }
}

impl Act {
    const TOUCHED: PlanePoint = PlanePoint::new(Pixels::new(200.0), Pixels::new(300.0));
    const SPREAD: Pixels = Pixels::new(120.0);

    fn touch_two_fingers(session: &Session) {
        SessionEvent::PairTouched {
            at: Self::TOUCHED,
            spread: Self::SPREAD,
        }
        .applied(session);
    }

    fn today_spot(session: &Session, step: StepNumber) -> Option<Spot> {
        session
            .day_at(session.now())
            .map(|day| Spot::new(day.number(), step))
    }

    fn held(session: &Session) -> Option<Gesture> {
        Some(session.gesture()).filter(|gesture| matches!(gesture, Gesture::Paired { .. }))
    }

    fn timer_fires_on(session: &Session, paired: Option<Gesture>) {
        if let Some(paired) = paired {
            SessionEvent::PairHeld(paired).applied(session);
        }
    }

    fn done_on(self, session: &Session) {
        match self {
            Act::CaptureAfterTwoDecisions => {
                SessionEvent::NoteDecision.applied(session);
                if let Some(earlier) = Self::today_spot(session, EARLIER) {
                    SessionEvent::PlaceHere(earlier).applied(session);
                }
                SessionEvent::NoteDecision.applied(session);
                SessionEvent::NoteDecision.applied(session);
                SessionEvent::CaptureReport.applied(session);
            }
            Act::CaptureWhileBrowsing => {
                if let Some(later) = Self::today_spot(session, LATER) {
                    session.browse(later);
                }
                SessionEvent::CaptureReport.applied(session);
            }
            Act::HoldTwoFingers => {
                Self::touch_two_fingers(session);
                Self::timer_fires_on(session, Self::held(session));
            }
            Act::MoveTwoFingers => {
                Self::touch_two_fingers(session);
                let start = Self::held(session);
                let far = Self::TOUCHED[guide::map_frame::Axis::Across] + Touch::Slop.extent();
                SessionEvent::TouchMoved(PlanePoint::new(
                    far,
                    Self::TOUCHED[guide::map_frame::Axis::Down],
                ))
                .applied(session);
                Self::timer_fires_on(session, start);
            }
            Act::SpreadOneFinger => {
                Self::touch_two_fingers(session);
                let held = Self::held(session);
                SessionEvent::PairSpread(Self::SPREAD + Touch::Slop.extent()).applied(session);
                Self::timer_fires_on(session, held);
            }
            Act::LiftTwoFingers => {
                Self::touch_two_fingers(session);
                let start = Self::held(session);
                SessionEvent::TouchEnded(Self::TOUCHED).applied(session);
                Self::timer_fires_on(session, start);
            }
            Act::HoldAgainAfterCapture => {
                Self::touch_two_fingers(session);
                let start = Self::held(session);
                Self::timer_fires_on(session, start);
                Self::timer_fires_on(session, start);
            }
            Act::CaptureAndExport => {
                SessionEvent::CaptureReport.applied(session);
                SessionEvent::ExportReports.applied(session);
            }
            Act::CaptureAndNote => {
                SessionEvent::CaptureReport.applied(session);
                session.note_report(
                    report::ReportNumber::new(1),
                    &ReportText::new("it froze".to_owned()),
                );
            }
        }
    }
}

#[derive(Deserialize)]
struct Exported {
    reports: Vec<Report>,
}

struct Shelf {
    home: HomeFolder,
}

impl Shelf {
    fn new() -> Shelf {
        Shelf {
            home: MemoryStorage::home(),
        }
    }

    fn export_file(&self) -> PathBuf {
        self.home.joined(["Documents", ReportStore::EXPORT_NAME])
    }

    fn store(&self) -> ReportStore {
        ReportStore::in_home(self.home.clone())
    }

    fn after(&self, act: Act) -> After {
        let trip = Drawing::Second.itinerary();
        let stage = Stage::on_schedule(&trip, WALKED)
            .unwrap()
            .drawn_with(Drawing::Second)
            .keeping_reports_in(self.store());
        Scene::after(stage, act)
    }

    fn only_report(after: &After) -> Report {
        match after.book.reports() {
            [report] => report.clone(),
            reports => panic!("expected one report, found {}", reports.len()),
        }
    }
}

#[test]
fn one_action_records_the_screen_the_step_the_position_and_the_tracker() {
    let shelf = Shelf::new();
    let after = shelf.after(Act::CaptureAfterTwoDecisions);
    let report = Shelf::only_report(&after);
    let capture = report.capture();
    assert_eq!(after.screen, Screen::Reports);
    assert_eq!(
        capture.fact(Fact::Screen),
        Some(&ReportText::new("map".to_owned()))
    );
    let step = capture.fact(Fact::Step).unwrap().to_string();
    assert!(step.contains("step"), "{step}");
    assert!(!step.contains("browsing"), "{step}");
    let position = capture.fact(Fact::Position).unwrap().to_string();
    assert!(position.contains("± 20 m"), "{position}");
    let tracker = capture.fact(Fact::Tracker).unwrap().to_string();
    assert!(tracker.contains("set 1"), "{tracker}");
    assert!(tracker.contains("along"), "{tracker}");
    assert!(capture.time().to_string().contains(':'));
    let [first, second] = capture.decisions() else {
        panic!("two decisions, not {:?}", capture.decisions());
    };
    assert!(first.to_string().contains("automatic"), "{first}");
    assert!(second.to_string().contains("set 1"), "{second}");
    assert_eq!(after.book.notice(), None);
    assert_eq!(ReportBook::opened(Some(shelf.store())).reports(), &[report]);
}

#[test]
fn a_report_says_which_step_was_being_browsed() {
    let after = Shelf::new().after(Act::CaptureWhileBrowsing);
    let report = Shelf::only_report(&after);
    let step = report.capture().fact(Fact::Step).unwrap().to_string();
    assert!(step.contains("step 2 browsing"), "{step}");
}

#[test]
fn two_fingers_held_still_capture_a_report_and_end_the_gesture() {
    let after = Shelf::new().after(Act::HoldTwoFingers);
    assert_eq!(after.book.reports().len(), 1);
    assert_eq!(after.gesture, Gesture::Idle);
}

#[test]
fn two_fingers_that_move_or_lift_capture_nothing() {
    assert_eq!(Shelf::new().after(Act::MoveTwoFingers).book.reports(), &[]);
    assert_eq!(Shelf::new().after(Act::SpreadOneFinger).book.reports(), &[]);
    assert_eq!(Shelf::new().after(Act::LiftTwoFingers).book.reports(), &[]);
}

#[test]
fn one_hold_captures_one_report_however_often_its_timer_fires() {
    assert_eq!(
        Shelf::new()
            .after(Act::HoldAgainAfterCapture)
            .book
            .reports()
            .len(),
        1
    );
}

#[test]
fn reports_reach_the_export_folder_only_through_export() {
    let captured = Shelf::new();
    let _kept = captured.after(Act::CaptureAfterTwoDecisions);
    assert!(captured.home.read(&captured.export_file()).is_err());
    let exporting = Shelf::new();
    let after = exporting.after(Act::CaptureAndExport);
    let text = exporting.home.read(&exporting.export_file()).unwrap();
    let exported: Exported = toml::from_str(&text).unwrap();
    assert_eq!(exported.reports, after.book.reports());
}

#[test]
fn a_typed_note_is_saved_with_its_report() {
    let shelf = Shelf::new();
    let after = shelf.after(Act::CaptureAndNote);
    let reopened = ReportBook::opened(Some(shelf.store()));
    assert_eq!(reopened.reports(), after.book.reports());
    assert_eq!(
        reopened.reports()[0].note(),
        &ReportText::new("it froze".to_owned())
    );
}

#[test]
fn a_page_with_no_reports_says_so_and_a_saved_note_shows_in_its_box() {
    let trip = Drawing::Second.itinerary();
    let wiki = Drawing::Second.wiki();
    let stages = Stage::all(&trip, &wiki);
    let page_of = |wanted: &str| {
        stages
            .iter()
            .find(|stage| stage.label().ends_with(wanted))
            .map(Harness::page)
            .unwrap()
    };
    let saved = page_of("bug reports page");
    assert!(saved.contains("near the bridge"), "{saved}");
    assert!(!saved.contains("No reports yet"), "{saved}");
    assert!(page_of("no bug reports yet").contains("No reports yet"));
}

#[test]
fn the_log_keeps_each_change_once_for_a_few_minutes() {
    let trip = Drawing::Second.itinerary();
    let day = trip.days().first().unwrap();
    let at = |hour| TripTime::on_date_at(day.zone(), day.date(), ClockTime::at_hour(hour));
    let decision = |now| {
        TrackerDecision::seen_in(
            &Guidance::new(day, LegProgress::default(), now, None),
            LegProgress::default(),
        )
    };
    let morning = at(6);
    let same_soon = morning.later_by(Minutes::new(1));
    let log = DecisionLog::default()
        .noted(morning, decision(morning))
        .unwrap();
    assert_eq!(log.noted(same_soon, decision(morning)), None);
    assert_eq!(log.texts_at(same_soon).len(), 1);
    assert_eq!(log.texts_at(morning.later_by(Minutes::new(6))).len(), 0);
    let late = at(20);
    let moved = log.noted(late, decision(late)).unwrap();
    assert_eq!(moved.texts_at(late).len(), 1);
    assert_ne!(decision(morning), decision(late));
}

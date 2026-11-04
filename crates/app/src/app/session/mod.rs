mod appearance;
mod dark_after_sunset;
pub mod gesture;
mod hand;
pub mod kept_file;
mod location_access;
#[cfg(target_os = "ios")]
mod location_feed;
mod map_health;
mod measure;
mod page_shift;
mod page_turn;
mod place;
mod ports;
pub mod progress_file;
mod session_event;
pub mod staged;
pub mod tracker_decision;
pub mod tracker_log;
pub mod trip_shelf;
mod wakes;
mod walked;
pub mod wall_clock;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use wiki::{EntryId, Wiki};

use dioxus::prelude::*;
use guide::Overview;
use guide::clock::TripTime;
use guide::day_number::DayNumber;
use guide::geo::{Bearing, Coordinate, Distance};
use guide::guidance::timeline::Halt;
use guide::guidance::{Guidance, LegProgress, Sighting};
use guide::itinerary::{Day, Itinerary};
use guide::readout::Trail;
use guide::{PlaceCatalog, Trip};
use layout::scale::Viewport;
use map_plan::{KeyEntry, MapLevel, MapMode, PickedMark, PinchWay};
use report::{ReportBook, ReportNumber, ReportStore, ReportText};
use storage_ports::HomeFolder;

use crate::app::history::{Fold, History, ScrollTop, StepFolds};
use crate::app::step::{DayPoint, Spot, Step, StepNumber, StepState, Toward};
use crate::app::visit::{Focus, Landing, Scroll, Visit};
use crate::app::visit::{Screen, Turning};
pub use appearance::Appearance;
pub use dark_after_sunset::DarkAfterSunset;
use gesture::Gesture;
pub use location_access::LocationAccess;
#[cfg(target_os = "ios")]
use location_feed::LocationFeed;
pub use map_health::MapHealth;
use measure::Cadence;
use menu::{Menu, PoseStyle, Pull};
pub use page_shift::PageShift;
use page_turn::PageTurn;
use place::Place;
use ports::LaunchArgs;
use progress_file::ProgressFile;
pub use session_event::SessionEvent;
use staged::Staged;
use tracker_log::DecisionLog;
use trip_shelf::TripPage;
pub use trip_shelf::TripShelf;
use wakes::Wakes;
use walked::Walked;
use wall_clock::WallClock;

#[derive(Clone)]
pub struct Session {
    itinerary: Arc<Itinerary>,
    places: Arc<PlaceCatalog>,
    overview: Arc<Overview>,
    wiki: Arc<Wiki>,
    screen: Signal<Screen>,
    visit: Signal<Visit>,
    history: Signal<History>,
    landing_top: Signal<Option<ScrollTop>>,
    touch_top: Signal<ScrollTop>,
    folds: Signal<StepFolds>,
    menu: Signal<Menu>,
    gesture: Signal<Gesture>,
    pull: Signal<Option<Pull>>,
    lean: Signal<Option<Toward>>,
    map_level: Signal<MapLevel>,
    map_mode: Signal<MapMode>,
    map_health: Signal<MapHealth>,
    key_shown: Signal<Vec<KeyEntry>>,
    picked: Signal<Option<PickedMark>>,
    viewport: Signal<Viewport>,
    position: Signal<Option<Sighting>>,
    spread: Signal<Option<Distance>>,
    heading: Signal<Option<Bearing>>,
    access: Signal<LocationAccess>,
    trail: Signal<Trail>,
    halt: Signal<Option<Halt>>,
    progress: Signal<LegProgress>,
    now: Signal<TripTime>,
    clock: WallClock,
    wakes: Signal<Wakes>,
    reports: Signal<ReportBook>,
    decisions: Signal<DecisionLog>,
    dark: Signal<DarkAfterSunset>,
    shelf: TripShelf,
}

impl PartialEq for Session {
    fn eq(&self, other: &Session) -> bool {
        Arc::ptr_eq(&self.itinerary, &other.itinerary)
    }
}

impl Session {
    pub fn open(trip: &Trip, shelf: TripShelf) -> Session {
        let wiki = use_hook(|| {
            Arc::new(Wiki::new(
                trip.itinerary(),
                trip.places().places(),
                trip.overview().landmarks(),
            ))
        });
        let itinerary = Arc::clone(trip.itinerary());
        let places = Arc::clone(trip.places());
        let overview = Arc::clone(trip.overview());
        let clock = WallClock::System(itinerary.head().zone());
        let now = use_signal(|| clock.time());
        let kept = use_hook(Place::kept);
        let walked = use_hook(|| Rc::new(Walked::kept()));
        let screen = use_signal(|| *kept.screen());
        let visit = use_signal(|| Visit::restored(kept.focus()));
        let history = use_signal(|| kept.history().clone());
        let today = itinerary.day_on(now.peek().trip_date());
        let home = shelf.home();
        let progress = use_signal(|| {
            home.as_ref()
                .map(ProgressFile::under)
                .map_or(kept.progress(), |file| {
                    file.restored(kept.progress(), today)
                })
        });
        let session = Session {
            places,
            overview,
            wiki,
            screen,
            visit,
            history,
            landing_top: use_signal(|| None),
            touch_top: use_signal(ScrollTop::default),
            folds: use_signal(StepFolds::default),
            menu: use_signal(Menu::default),
            gesture: use_signal(Gesture::default),
            pull: use_signal(|| None),
            lean: use_signal(|| None),
            map_level: use_signal(MapLevel::default),
            map_mode: use_signal(MapMode::default),
            map_health: use_signal(|| MapHealth::Drawing),
            key_shown: use_signal(Vec::new),
            picked: use_signal(|| None),
            viewport: use_signal(Viewport::default),
            position: use_signal(|| None),
            spread: use_signal(|| None),
            heading: use_signal(|| None),
            access: use_signal(LocationAccess::default),
            trail: use_signal(|| walked.trail().clone()),
            halt: use_signal(|| walked.halt()),
            progress,
            now,
            clock,
            wakes: use_signal(Wakes::default),
            reports: use_signal(|| ReportBook::opened(home.clone().map(ReportStore::in_home))),
            decisions: use_signal(|| walked.decisions().clone()),
            dark: use_signal(|| DarkAfterSunset::restored_in(home.as_ref())),
            itinerary,
            shelf,
        };
        let keeper = session.clone();
        use_hook(|| keeper.keep_time());
        let watcher = session.clone();
        use_hook(|| watcher.await_wake(document::eval(Wakes::LISTENER)));
        #[cfg(target_os = "ios")]
        LocationFeed::follow(&session);
        use_effect(move || {
            Place::new(
                *screen.read(),
                visit.read().focus(),
                *progress.read(),
                history.read().clone(),
            )
            .keep();
        });
        let trip = Arc::clone(&session.itinerary);
        let written = use_hook(|| Rc::new(Cell::new(None)));
        use_effect(move || {
            now.read();
            let latest = *progress.peek();
            ProgressFile::keep_if_changed(shelf.home().as_ref(), &written, latest, &trip);
        });
        let tracker = session.clone();
        use_effect(move || SessionEvent::TrackWalk.applied(&tracker));
        let trail = session.trail;
        let halt = session.halt;
        let decisions = session.decisions;
        use_effect(move || {
            Walked::new(trail.read().clone(), *halt.read(), decisions.read().clone()).keep();
        });
        let noting = session.clone();
        use_effect(move || SessionEvent::NoteDecision.applied(&noting));
        let confirming = session.clone();
        use_effect(move || SessionEvent::ConfirmProgress.applied(&confirming));
        session
    }

    #[must_use]
    pub fn launch_shelf(page: TripPage) -> TripShelf {
        let shelf = TripShelf::open(LaunchArgs::trip_path(), HomeFolder::of_user(), page);
        use_hook(|| shelf.listen(document::eval(TripShelf::LISTENER)));
        shelf
    }

    pub fn forget_trip(home: Option<&HomeFolder>) {
        Place::default().keep();
        Walked::default().keep();
        home.map(ProgressFile::under)
            .into_iter()
            .for_each(|file| file.forget());
    }

    #[must_use]
    pub fn shelf(&self) -> TripShelf {
        self.shelf
    }

    pub fn keep_time(self) {
        spawn(async move {
            tokio::time::sleep(Cadence::Clock.every()).await;
            self.catch_up_clock();
            self.keep_time();
        });
    }

    #[expect(
        clippy::must_use_candidate,
        reason = "the clock moves as it is read; most callers only want it moved"
    )]
    pub fn catch_up_clock(&self) -> Option<TripTime> {
        let mut shown = self.now;
        let current = self.now();
        self.clock
            .moved_on(current)
            .inspect(|real| shown.set(*real))
    }

    pub fn staged(
        itinerary: Arc<Itinerary>,
        places: Arc<PlaceCatalog>,
        overview: Arc<Overview>,
        stage: &impl Staged,
    ) -> Session {
        Session {
            itinerary,
            places,
            overview,
            wiki: stage.wiki(),
            screen: use_signal(|| *stage.screen()),
            visit: use_signal(|| Visit::default().jumped(stage.focus(), stage.landing())),
            history: use_signal(|| stage.starting_history()),
            landing_top: use_signal(|| None),
            touch_top: use_signal(ScrollTop::default),
            folds: use_signal(StepFolds::default),
            menu: use_signal(|| stage.menu()),
            gesture: use_signal(Gesture::default),
            pull: use_signal(|| None),
            lean: use_signal(|| None),
            map_level: use_signal(|| stage.map_level()),
            map_mode: use_signal(|| stage.map_mode()),
            map_health: use_signal(|| MapHealth::Drawing),
            key_shown: use_signal(|| stage.key_shown()),
            picked: use_signal(|| stage.picked()),
            viewport: use_signal(|| stage.viewport()),
            position: use_signal(|| stage.position().map(Sighting::Fresh)),
            spread: use_signal(|| stage.spread()),
            heading: use_signal(|| stage.heading()),
            access: use_signal(|| stage.access()),
            trail: use_signal(|| stage.trail().iter().copied().collect()),
            halt: use_signal(|| stage.halt()),
            progress: use_signal(|| stage.progress()),
            now: use_signal(|| stage.now()),
            clock: stage.clock(),
            wakes: use_signal(Wakes::default),
            reports: use_signal(|| ReportBook::opened(stage.reports())),
            decisions: use_signal(DecisionLog::default),
            dark: use_signal(DarkAfterSunset::default),
            shelf: TripShelf::staged(stage.refusal()),
        }
    }

    #[must_use]
    pub fn progress(&self) -> LegProgress {
        *self.progress.peek()
    }

    #[must_use]
    pub fn language(&self) -> guide::Language {
        self.itinerary.head().language()
    }

    #[must_use]
    pub fn itinerary(&self) -> &Itinerary {
        &self.itinerary
    }

    #[must_use]
    pub fn places(&self) -> &PlaceCatalog {
        &self.places
    }

    #[must_use]
    pub fn wiki(&self) -> &Wiki {
        &self.wiki
    }

    #[must_use]
    pub fn overview(&self) -> &Overview {
        &self.overview
    }

    #[must_use]
    pub fn screen(&self) -> Screen {
        *self.screen.read()
    }

    pub fn show(&self, screen: Screen) {
        let mut current = self.screen;
        let mut visit = self.visit;
        let before = *visit.peek();
        let reopened = before.scrolled(Scroll::Follow);
        (screen == Screen::Today && *current.peek() != Screen::Today && reopened != before)
            .then(|| visit.set(reopened));
        current.set(screen);
    }

    pub fn leave(&self, top: ScrollTop) {
        if self.history.peek().can_turn(Toward::Previous).is_some() {
            SessionEvent::TurnHistory {
                toward: Toward::Previous,
                top,
            }
            .applied(self);
        } else {
            self.show(Screen::Today);
        }
    }

    #[must_use]
    pub fn history(&self) -> History {
        self.history.read().clone()
    }

    #[must_use]
    pub fn landing_top(&self) -> Option<ScrollTop> {
        *self.landing_top.read()
    }

    #[must_use]
    pub fn reader(&self) -> Option<EntryId> {
        self.screen.read().wiki_entry()
    }

    pub fn jump(&self, day: DayNumber, landing: Landing) {
        let mut visit = self.visit;
        let focus = self
            .itinerary
            .day_number(day)
            .and_then(|day| landing.spot_in(day))
            .map_or(Focus::Now, |spot| Focus::toward(spot, self.now_spot()));
        let jumped = visit.peek().jumped(focus, landing);
        visit.set(jumped);
        SessionEvent::CloseMenu.applied(self);
        self.show(Screen::Today);
    }

    #[must_use]
    pub fn stepped_focus(&self, toward: Toward) -> Option<Spot> {
        let shown = self.visit().focus().spot();
        Some(*self.map_level.peek())
            .filter(|level| *level != MapLevel::Leg && *self.screen.peek() == Screen::Map)
            .map_or_else(
                || self.stepped_leg(shown, toward),
                |level| {
                    shown.or_else(|| self.now_spot())?.stepped_across(
                        level,
                        toward,
                        &self.itinerary,
                    )
                },
            )
    }

    fn stepped_leg(&self, shown: Option<Spot>, toward: Toward) -> Option<Spot> {
        shown.map_or_else(
            || {
                self.day_at(self.now()).and_then(|today| {
                    DayPoint::locate(&self.guidance(today), &Step::of_day(today)).stepped(
                        today,
                        toward,
                        &self.itinerary,
                    )
                })
            },
            |spot| spot.stepped(toward, &self.itinerary),
        )
    }

    pub fn browse(&self, spot: Spot) {
        let mut visit = self.visit;
        let browsed = visit.peek().jumped(
            Focus::toward(spot, self.now_spot()),
            Landing::Step(spot.step()),
        );
        visit.set(browsed);
    }

    #[must_use]
    pub fn visit(&self) -> Visit {
        *self.visit.read()
    }

    #[must_use]
    pub fn map_level(&self) -> MapLevel {
        *self.map_level.read()
    }

    #[must_use]
    pub fn map_mode(&self) -> MapMode {
        *self.map_mode.read()
    }

    #[must_use]
    pub fn map_health(&self) -> MapHealth {
        *self.map_health.read()
    }

    #[must_use]
    pub fn key_shown(&self) -> Vec<KeyEntry> {
        self.key_shown.read().clone()
    }

    #[must_use]
    pub fn picked(&self) -> Option<PickedMark> {
        self.picked.read().clone()
    }

    pub fn show_key(&self, mut shown: Vec<KeyEntry>) {
        shown.sort_unstable();
        let mut key = self.key_shown;
        key.set(shown);
    }

    pub fn pick_mark(&self, mark: Option<PickedMark>) {
        let mut picked = self.picked;
        picked.set(mark);
    }

    pub fn pinch_map(&self, way: PinchWay) {
        let mut level = self.map_level;
        let pinched = level.peek().pinched(way);
        level.set(pinched);
    }

    pub fn note_map_failed(&self) {
        let mut health = self.map_health;
        health.set(MapHealth::Failed);
    }

    #[must_use]
    pub fn viewport(&self) -> Viewport {
        *self.viewport.read()
    }

    pub fn measure_viewport(&self, measure: impl FnOnce(Viewport) -> Viewport) {
        let mut current = self.viewport;
        let previous = *current.peek();
        let measured = measure(previous);
        (previous != measured).then(|| current.set(measured));
    }

    #[must_use]
    pub fn dark_after_sunset(&self) -> DarkAfterSunset {
        *self.dark.read()
    }

    #[must_use]
    pub fn appearance(&self) -> Appearance {
        let now = self.now();
        Appearance::chosen(self.dark_after_sunset(), self.day_at(now), now)
    }

    #[must_use]
    pub fn now(&self) -> TripTime {
        *self.now.read()
    }

    #[must_use]
    pub fn heading(&self) -> Option<Bearing> {
        *self.heading.read()
    }

    #[must_use]
    pub fn location_access(&self) -> LocationAccess {
        *self.access.read()
    }

    #[must_use]
    pub fn gesture(&self) -> Gesture {
        *self.gesture.peek()
    }

    #[must_use]
    pub fn report_book(&self) -> ReportBook {
        self.reports.read().clone()
    }

    pub fn note_report(&self, number: ReportNumber, note: &ReportText) {
        let mut reports = self.reports;
        let noted = reports.peek().clone().noted(number, note);
        reports.set(noted);
    }

    #[must_use]
    pub fn trail(&self) -> Vec<Coordinate> {
        self.trail.read().points().to_vec()
    }

    #[must_use]
    pub fn day(&self) -> Option<&Day> {
        self.visit
            .read()
            .focus()
            .day()
            .and_then(|number| self.itinerary.day_number(number))
            .or_else(|| self.day_at(self.now()))
    }

    #[must_use]
    pub fn fold(&self, day: DayNumber, step: StepNumber, state: StepState) -> Fold {
        self.folds.read().fold(day, step, state)
    }

    pub fn toggle_step(&self, day: DayNumber, step: StepNumber) {
        let mut visit = self.visit;
        let focused = visit
            .peek()
            .refocused(Focus::toward(Spot::new(day, step), self.now_spot()));
        visit.set(focused);
        let mut folds = self.folds;
        let toggled = folds.peek().toggled(day, step);
        folds.set(toggled);
    }

    #[must_use]
    pub fn halted(&self) -> Option<Halt> {
        let now = self.now();
        (*self.halt.read()).filter(|halt| halt.still_for(now).is_some())
    }

    pub fn guidance<'day>(&self, day: &'day Day) -> Guidance<'day> {
        let progress = *self.progress.read();
        let now = self.now();
        let today = self.day_at(now).map(Day::number);
        let sighting = *self.position.read();
        let halt = (*self.halt.read())
            .filter(|_| today == Some(day.number()))
            .filter(|_| sighting.and_then(Sighting::fresh).is_some());
        sighting
            .filter(|_| today == Some(day.number()))
            .map_or_else(
                || Guidance::new(day, progress, now, None),
                |sighting| sighting.guidance(day, progress, now),
            )
            .held(progress)
            .paused(halt)
    }

    #[must_use]
    pub fn menu(&self) -> Menu {
        *self.menu.read()
    }

    #[must_use]
    pub fn menu_style(&self) -> PoseStyle {
        self.menu().style(*self.pull.read(), self.viewport())
    }

    #[must_use]
    pub fn page_shift(&self) -> PageShift {
        PageShift::new(*self.lean.read(), self.viewport())
    }

    #[must_use]
    pub fn neighbour_day(&self, day: DayNumber, toward: Toward) -> Option<&Day> {
        let days = self.itinerary.days();
        let at = days.iter().position(|each| each.number() == day)?;
        match toward {
            Toward::Previous => at.checked_sub(1).and_then(|before| days.get(before)),
            Toward::Next => days.get(at + 1),
        }
    }

    pub fn page_turn(&self, toward: Toward) -> Option<PageTurn> {
        let turning = match self.screen.peek().turning() {
            Turning::Steps if *self.map_mode.peek() == MapMode::Explore => Turning::Still,
            turning => turning,
        };
        match turning {
            Turning::Steps => self.stepped_focus(toward).map(PageTurn::Step),
            Turning::Days => self
                .day()
                .and_then(|day| self.neighbour_day(day.number(), toward))
                .map(|day| PageTurn::Day(day.number())),
            Turning::History => self.history.peek().can_turn(toward).map(PageTurn::History),
            Turning::Still => None,
        }
    }

    pub fn turn_page(&self, toward: Toward) {
        self.page_turn(toward)
            .into_iter()
            .for_each(|turn| turn.show(self));
    }

    pub fn lean_toward(&self, toward: Option<Toward>) {
        let mut lean = self.lean;
        let previous = *lean.peek();
        (previous != toward).then(|| lean.set(toward));
    }

    fn await_wake(self, mut listener: document::Eval) {
        spawn(async move {
            listener.recv::<bool>().await.is_ok().then(move || {
                self.catch_up_clock();
                let mut wakes = self.wakes;
                let next = wakes.peek().next();
                wakes.set(next);
                self.await_wake(listener);
            });
        });
    }

    fn now_spot(&self) -> Option<Spot> {
        self.day_at(self.now()).and_then(|today| {
            DayPoint::locate(&self.guidance(today), &Step::of_day(today))
                .current()
                .map(|step| Spot::new(today.number(), step))
        })
    }

    #[must_use]
    pub fn day_at(&self, now: TripTime) -> Option<&Day> {
        let days = self.itinerary.days();
        self.itinerary.day_on(now.trip_date()).or_else(|| {
            days.first()
                .filter(|first| first.date() > now.trip_date())
                .or_else(|| days.last())
        })
    }
}

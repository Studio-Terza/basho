mod situation;
mod walk_scene;
mod wanted;

use basho::app::Position;
use std::borrow::Cow;
use std::time::Duration;

use dioxus::prelude::*;
use std::fmt;

use guide::geo::{Bearing, Coordinate, Degrees, Distance, Share};

use guide::BufferState;
use guide::guidance::timeline::Halt;

use guide::clock::{ClockTime, Minutes, TripTime};

use guide::itinerary::{Day, Itinerary};

use guide::end::End;

use guide::leg::{Leg, LegIndex, Mode};

use guide::guidance::LegProgress;

use basho::app::history::{History, Mark, ScrollTop};
use guide::words::Spelling;

use basho::app::history::StepFolds;
use basho::app::session::LocationAccess;
use basho::app::session::staged::Staged;
use basho::app::session::trip_shelf::{PickedFile, TripRefusal, TripText};
use basho::app::session::wall_clock::WallClock;
use basho::app::step::{GoalNumber, Step, StepNumber};
use basho::app::visit::Screen;
use basho::app::visit::{Focus, Landing, Visit};
use layout::scale::Viewport;
use map_plan::{KeyEntry, MapLevel, MapMode, PickedMark};
use menu::Menu;
use report::{Report, ReportStore};
use situation::Situation;
use std::sync::Arc;
use storage_fake::MemoryStorage;
use walk_scene::WalkScene;
use wanted::Wanted;
use wiki::Wiki;

use crate::ui_check::{Drawing, Handset};

#[derive(Props, Clone, PartialEq)]
pub struct Stage {
    screen: Screen,
    focus: Focus,
    landing: Landing,
    menu: Menu,
    position: Option<Coordinate>,
    heading: Option<Bearing>,
    access: LocationAccess,
    trail: Vec<Coordinate>,
    now: TripTime,
    progress: LegProgress,
    halt: Option<Halt>,
    extra: Vec<&'static str>,
    situation: Situation,
    keyed: Keyed,
    drawing: Drawing,
    label: String,
    clock: Option<TripTime>,
    origin: Option<Screen>,
    reports: Option<ReportStore>,
    refused: Option<PickedFile>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Keyed {
    Plain,
    Tapped,
    Exploring,
}

impl fmt::Display for Stage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}

impl Stage {
    const OFF_ROUTE: Distance = Distance::new(400.0);
    const SPREAD: Distance = Distance::new(20.0);
    const KEY: [KeyEntry; 3] = [KeyEntry::RoadMajor, KeyEntry::Goal, KeyEntry::PoiKonbini];
    const SAVED_REPORTS: [&str; 2] = [
        include_str!("../saved_reports/report-0001.toml"),
        include_str!("../saved_reports/report-0002.toml"),
    ];
    const WEEK: Duration = Duration::from_hours(7 * 24);
    const FACING_SOUTH_WEST: Degrees = Degrees::new(225.0);
    const TRAIL_STEP: Distance = Distance::new(40.0);
    const TRAIL_LEN: usize = 6;
    const ALWAYS: [&str; 1] = ["nav.tabs .tab.active"];
    const LOCATION_PROBLEMS: [LocationAccess; 5] = [
        LocationAccess::Asking,
        LocationAccess::Denied,
        LocationAccess::ServicesOff,
        LocationAccess::Approximate,
        LocationAccess::Restricted,
    ];
    const MAP_KEY: [&str; 2] = [".map-key summary", ".map-picked"];
    const MAP_NOW: [&str; 7] = [
        ".top-line .step-title",
        ".top-line .time-check",
        ".top-row .step-back",
        ".top-row .step-next",
        ".day-map .map-slot",
        ".status-bar .status",
        ".status-bar .arrow",
    ];
    const TODAY_NOW: [&str; 4] = [
        ".day-nav .day-title",
        ".day-nav .days-open",
        ".day-head .leave",
        ".steps .step .step-head",
    ];
    const WIKI: [&str; 3] = [
        ".wiki-bar .wiki-back",
        "article.wiki header h1",
        "article.wiki header .wiki-kind",
    ];
    const REPORTS: [&str; 3] = [
        ".reports .place-back",
        ".reports h1",
        ".reports .report-export",
    ];
    const SAVED_REPORT: [&str; 2] = [
        ".report-list .report h2",
        ".report-list .report .report-note",
    ];
    const NO_REPORT: [&str; 1] = [".reports .reports-empty"];
    const BACK_CHIP: [&str; 1] = [".screen .back-chip"];
    const MAP_BROWSING: [&str; 7] = [
        ".browse-banner .browse-title .browse-what",
        ".browse-banner .browse-title .step-title",
        ".browse-banner .step-back",
        ".browse-banner .back-to-now",
        ".browse-banner .step-next",
        ".day-map .map-slot",
        ".map-screen.browsing:not(:has(.buffer, .top-line, .recovery, .status-bar, .arrow))",
    ];
    const TODAY_BROWSING: [&str; 3] = [
        ".day-nav .day-title",
        ".today .back-to-now",
        ".steps .step.focused .step-head",
    ];
    const BUFFER_STATES: [BufferState; 4] = [
        BufferState::Comfortable,
        BufferState::Tight,
        BufferState::Short,
        BufferState::Comfortable,
    ];
    const BUFFER_SITUATIONS: [Situation; 4] = [
        Situation::BufferComfortable,
        Situation::BufferTight,
        Situation::BufferShort,
        Situation::BufferCutoff,
    ];
    const BUFFER_REQUIRED: [&[&str]; 4] = [
        &Self::COMFORTABLE,
        &Self::TIGHT,
        &Self::SHORT,
        &Self::CUTOFF,
    ];
    const BUFFER_WANTED: [Wanted; 4] = [
        Wanted::Ride,
        Wanted::Ride,
        Wanted::RideWithBackup,
        Wanted::Cutoff,
    ];
    const LATE: Minutes = Minutes::new(40);
    const BEFORE_RIDE: Minutes = Minutes::new(-5);
    const AFTER_START: Minutes = Minutes::new(1);
    const SEEN_BEFORE: Minutes = Minutes::new(1);
    const SECOND_GOAL: GoalNumber = GoalNumber::from_position(Position::new(1));
    const TRIP: [&str; 4] = [
        ".trip-view header h1",
        ".trip-picker .trip-open",
        ".trip-picker .trip-built-in",
        ".trip-view .place-back",
    ];
    const REFUSED: [&str; 1] = [".trip-picker .trip-refusal"];
    const REFUSED_FILE: &str = "not-a-trip.toml";
    const REFUSED_TEXT: &str = "[trip]\nname = \"No days\"\nunknown = 1\n";
    const MENU_SELECTORS: [&str; 9] = [
        ".menu .menu-tools .menu-index",
        ".menu .menu-tools .report-bug",
        ".menu .menu-tools .reports-open",
        ".menu .day-row",
        ".menu .menu-head .menu-trip",
        ".menu .menu-day.current .day-row .mark",
        ".menu .day-row .day-what ruby rt",
        ".menu .goal-row .goal-n",
        ".menu .goal-row .by",
    ];
    const COMFORTABLE: [&str; 3] = [
        ".buffer.ride.comfortable .deadline ruby rt",
        ".buffer.comfortable .spare:not(.inverted)",
        ".top-line .time-check",
    ];
    const TIGHT: [&str; 1] = [".buffer.tight .spare.inverted"];
    const SHORT: [&str; 3] = [
        ".buffer.short .spare.inverted",
        ".buffer.short .buffer-advice",
        ".buffer.short .if-missed",
    ];
    const CUTOFF: [&str; 2] = [".buffer.cutoff .deadline", ".buffer.cutoff .spare"];
    const STOPPED: [&str; 3] = [
        ".status.stopped ruby rt",
        ".status.stopped .stop-time",
        ".buffer .spare",
    ];
    const LOCATION_OFF: [&str; 1] = [".status-bar .status.location-off"];
    const UNFOUND: [&str; 1] = [".buffer.found-by-a-scene"];
    const GOAL_SPARE: [&str; 2] = [".goal .goal-spare .deadline", ".goal .goal-spare .spare"];
    const LIVE_LATE: [&str; 2] = [".live-line .live-now", ".live-line .late"];
    const LIVE_SHORT: [&str; 2] = [".live-line .live-now", ".live-line .spare.inverted"];

    pub fn all(itinerary: &Itinerary, wiki: &Wiki) -> Vec<Stage> {
        let first = itinerary.days().first();
        let days = itinerary.days();
        let menus = days.iter().flat_map(|day| {
            let time = day
                .legs()
                .first()
                .map_or(ClockTime::MIDNIGHT, |leg| leg.time(End::Start))
                .later_by(Self::AFTER_START);
            [
                Self::timed(day, Screen::Today, time, Situation::MenuOpen).with_menu(),
                Self::timed(day, Screen::Map, time, Situation::MenuOpen).with_menu(),
            ]
        });
        let browsing = days
            .iter()
            .zip(days.iter().skip(1))
            .flat_map(|(day, next)| {
                let time = day
                    .legs()
                    .first()
                    .map_or(ClockTime::MIDNIGHT, |leg| leg.time(End::Start));
                [
                    Self::timed(day, Screen::Today, time, Situation::BrowsingNextDay)
                        .browsing_at(next, Landing::Top),
                    Self::timed(next, Screen::Today, time, Situation::BrowsingDayBefore)
                        .browsing_at(day, Landing::Top),
                    Self::timed(day, Screen::Map, time, Situation::BrowsingMidDay)
                        .titled(&format!("of day {}", next.number()))
                        .browsing_at(next, Self::middle_of(next)),
                ]
            });
        let jumps = days
            .iter()
            .zip(days.iter().skip(1))
            .filter(|(_, next)| next.goals().len() > 1)
            .flat_map(|(day, next)| {
                let time = day
                    .legs()
                    .first()
                    .map_or(ClockTime::MIDNIGHT, |leg| leg.time(End::Start));
                [Screen::Today, Screen::Map].map(|screen| {
                    Self::timed(day, screen, time, Situation::MenuJump)
                        .titled(&format!("to day {} goal 2", next.number()))
                        .browsing_at(next, Landing::Goal(Self::SECOND_GOAL))
                })
            });
        let trip_pages = first.into_iter().flat_map(|day| {
            let opens = day
                .legs()
                .first()
                .map_or(ClockTime::MIDNIGHT, |leg| leg.time(End::Start));
            [
                Self::timed(day, Screen::Trip, opens, Situation::TripPicker),
                Self::timed(day, Screen::Trip, opens, Situation::TripRefused)
                    .requiring(&Self::REFUSED)
                    .refused(PickedFile::new(
                        Spelling::new(Self::REFUSED_FILE.to_owned()),
                        Some(TripText::new(Cow::Borrowed(Self::REFUSED_TEXT))),
                    )),
            ]
        });
        let blocked = first.into_iter().flat_map(|day| {
            let time = day
                .legs()
                .first()
                .map_or(ClockTime::MIDNIGHT, |leg| leg.time(End::Start))
                .later_by(Self::AFTER_START);
            Self::LOCATION_PROBLEMS.map(|access| {
                Self::timed(day, Screen::Map, time, Situation::LocationProblem)
                    .titled(&format!("{access:?}"))
                    .blocked(access)
                    .requiring(&Self::LOCATION_OFF)
            })
        });
        days.iter()
            .flat_map(Self::day_stages)
            .chain(blocked)
            .chain(menus)
            .chain(browsing)
            .chain(jumps)
            .chain(Self::wiki_stages(first, wiki))
            .chain(trip_pages)
            .chain(Self::buffer_stages(itinerary))
            .collect()
    }

    #[cfg(test)]
    pub fn on_screen(day: &Day, screen: Screen) -> Stage {
        Self::timed(day, screen, ClockTime::MIDNIGHT, Situation::WikiPage)
    }

    fn wiki_stages(first: Option<&Day>, wiki: &Wiki) -> Vec<Stage> {
        let pages = wiki.pages();
        let wiki_pages = first.into_iter().flat_map(|day| {
            let opens = day
                .legs()
                .first()
                .map_or(ClockTime::MIDNIGHT, |leg| leg.time(End::Start));
            pages
                .iter()
                .map(move |id| Self::timed(day, Screen::Wiki(*id), opens, Situation::WikiPage))
        });
        let chips = first.into_iter().zip(pages.first()).flat_map(|(day, id)| {
            let opens = day
                .legs()
                .first()
                .map_or(ClockTime::MIDNIGHT, |leg| leg.time(End::Start));
            [Screen::Today, Screen::Map].map(|screen| {
                Self::timed(day, screen, opens, Situation::CameFromWiki)
                    .came_from(Screen::Wiki(*id))
            })
        });
        wiki_pages.chain(chips).collect()
    }

    pub fn buffer_stages(itinerary: &Itinerary) -> Vec<Stage> {
        let scenes = WalkScene::all(itinerary);
        let first = itinerary.days().first();
        let found = Self::BUFFER_STATES
            .into_iter()
            .zip(Self::BUFFER_SITUATIONS)
            .zip(Self::BUFFER_REQUIRED)
            .zip(Self::BUFFER_WANTED)
            .filter_map(|(((state, situation), required), wanted)| {
                WalkScene::first(&scenes, state, wanted)
                    .map(|scene| Self::walking(&scene, Screen::Map, situation).requiring(required))
                    .or_else(|| Self::unfound(first, situation))
            });
        let goals = WalkScene::first(&scenes, BufferState::Comfortable, Wanted::Ride)
            .map(|scene| {
                Self::walking(&scene, Screen::Today, Situation::GoalBuffers)
                    .requiring(&Self::GOAL_SPARE)
            })
            .or_else(|| Self::unfound(first, Situation::GoalBuffers));
        let browsing = WalkScene::first(&scenes, BufferState::Short, Wanted::RideWithBackup)
            .map(|scene| {
                Self::walking(&scene, Screen::Map, Situation::BrowsingWhileShort)
                    .browsing_at(scene.day(), Self::middle_of(scene.day()))
                    .requiring(&Self::LIVE_SHORT)
            })
            .or_else(|| Self::unfound(first, Situation::BrowsingWhileShort));
        let stopped = scenes
            .iter()
            .find_map(|scene| {
                scene.stopped().map(|halt| {
                    Self::timed(
                        scene.day(),
                        Screen::Map,
                        scene
                            .leg()
                            .time(End::Finish)
                            .later_by(WalkScene::STOPPED_FOR),
                        Situation::Stopped,
                    )
                    .standing_at(scene.leg().point(End::Finish))
                    .confirmed(scene.progress())
                    .halted(halt)
                    .requiring(&Self::STOPPED)
                })
            })
            .or_else(|| Self::unfound(first, Situation::Stopped));
        let reports = first.into_iter().flat_map(|day| {
            [
                Self::timed(
                    day,
                    Screen::Reports,
                    ClockTime::MIDNIGHT,
                    Situation::ReportsPage,
                )
                .keeping_reports_in(Self::saved_reports())
                .requiring(&Self::SAVED_REPORT),
                Self::timed(
                    day,
                    Screen::Reports,
                    ClockTime::MIDNIGHT,
                    Situation::ReportsEmpty,
                )
                .requiring(&Self::NO_REPORT),
            ]
        });
        let keyed = first.into_iter().flat_map(|day| {
            [
                Stage {
                    keyed: Keyed::Tapped,
                    ..Self::timed(day, Screen::Map, ClockTime::MIDNIGHT, Situation::MapKey)
                }
                .requiring(&Self::MAP_KEY),
                Self::timed(day, Screen::Map, ClockTime::MIDNIGHT, Situation::Exploring)
                    .exploring(),
            ]
        });
        found
            .chain(goals)
            .chain(browsing)
            .chain(stopped)
            .chain(reports)
            .chain(keyed)
            .collect()
    }

    pub fn unfound(day: Option<&Day>, situation: Situation) -> Option<Stage> {
        day.map(|day| {
            Self::timed(day, Screen::Map, ClockTime::MIDNIGHT, situation)
                .titled("no scene shows this state")
                .requiring(&Self::UNFOUND)
        })
    }

    fn walking(scene: &WalkScene<'_>, screen: Screen, situation: Situation) -> Stage {
        Self::timed(scene.day(), screen, scene.time(), situation)
            .walking_on(scene.leg())
            .confirmed(scene.progress())
    }

    pub fn exploring(self) -> Stage {
        Stage {
            keyed: Keyed::Exploring,
            ..self
        }
    }

    pub fn drawing(&self) -> Drawing {
        self.drawing
    }

    pub fn drawn_with(self, drawing: Drawing) -> Stage {
        Stage { drawing, ..self }
    }

    pub fn blocked(self, access: LocationAccess) -> Stage {
        Stage { access, ..self }
    }

    pub fn halted(self, halt: Halt) -> Stage {
        Stage {
            halt: Some(halt),
            ..self
        }
    }

    pub fn refused(self, picked: PickedFile) -> Stage {
        Stage {
            refused: Some(picked),
            ..self
        }
    }

    pub fn requiring(self, extra: &[&'static str]) -> Stage {
        Stage {
            extra: extra.to_vec(),
            ..self
        }
    }

    pub fn standing_at(self, at: Coordinate) -> Stage {
        Stage {
            position: Some(at),
            trail: vec![at],
            ..self
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn required(&self) -> Vec<&'static str> {
        let menu = (self.menu != Menu::CLOSED)
            .then_some(Self::MENU_SELECTORS.iter())
            .into_iter()
            .flatten();
        let browsing = self.focus.spot();
        let screen: &[&str] = match self.screen {
            Screen::Map => browsing.map_or(&Self::MAP_NOW[..], |_| &Self::MAP_BROWSING[..]),
            Screen::Today => browsing.map_or(&Self::TODAY_NOW[..], |_| &Self::TODAY_BROWSING[..]),
            Screen::Wiki(_) => &Self::WIKI,
            Screen::Reports => &Self::REPORTS,
            Screen::Trip => &Self::TRIP,
        };
        let chip = self.origin.iter().flat_map(|_| Self::BACK_CHIP.iter());
        Self::ALWAYS
            .iter()
            .chain(screen)
            .chain(chip)
            .chain(menu)
            .chain(self.extra.iter())
            .copied()
            .collect()
    }

    pub fn came_from(self, origin: Screen) -> Stage {
        Stage {
            origin: Some(origin),
            ..self
        }
    }

    fn saved_reports() -> ReportStore {
        let store = ReportStore::in_home(MemoryStorage::home());
        Self::SAVED_REPORTS
            .iter()
            .map(|text| toml::from_str::<Report>(text).expect("a saved report"))
            .for_each(|report| store.keep(&report).expect("a kept report"));
        store
    }

    pub fn keeping_reports_in(self, reports: ReportStore) -> Stage {
        Stage {
            reports: Some(reports),
            ..self
        }
    }

    pub fn with_menu(self) -> Stage {
        Stage {
            menu: Menu::OPEN,
            ..self
        }
    }

    pub fn browsing_at(self, day: &Day, landing: Landing) -> Stage {
        Stage {
            focus: landing.spot_in(day).map_or(Focus::Now, Focus::Browsing),
            landing,
            ..self
        }
    }

    pub fn middle_of(day: &Day) -> Landing {
        let steps = Step::of_day(day).len();
        Landing::Step(StepNumber::from_position(Position::new(
            (0..steps)
                .zip((0..steps).rev())
                .filter(|(front, back)| front < back)
                .count(),
        )))
    }

    pub fn titled(self, what: &str) -> Stage {
        Stage {
            label: format!("{} · {what}", self.label),
            ..self
        }
    }

    pub fn off_route_from(self, start: &Coordinate) -> Stage {
        Stage {
            position: Some(start.offset_by(Self::OFF_ROUTE, Distance::default())),
            ..self
        }
    }

    pub fn walking_on(self, leg: &Leg) -> Stage {
        let middle = leg.line().length() * Share::HALF;
        let fixes: Vec<Coordinate> = (0..Self::TRAIL_LEN)
            .rev()
            .filter_map(|back| {
                let behind = Self::TRAIL_STEP * f64::from(u32::try_from(back).unwrap_or_default());
                leg.line()
                    .point_along((middle - behind).max(Distance::default()))
            })
            .collect();
        Stage {
            position: fixes.last().copied(),
            heading: Some(Bearing::new(Self::FACING_SOUTH_WEST.degrees())),
            trail: fixes,
            ..self
        }
    }

    pub fn confirmed(self, progress: LegProgress) -> Stage {
        Stage { progress, ..self }
    }

    #[cfg(test)]
    pub fn clocked_at(self, clock: TripTime) -> Stage {
        Stage {
            clock: Some(clock),
            ..self
        }
    }

    pub fn ticked_before(self, gap: Duration) -> Stage {
        Stage {
            now: self.now.earlier_by(gap),
            ..self
        }
    }

    fn timed(day: &Day, screen: Screen, time: ClockTime, situation: Situation) -> Stage {
        Stage {
            label: format!(
                "day {} · {} · {screen:?} · {situation}",
                day.number(),
                time.since_midnight()
            ),
            situation,
            keyed: Keyed::Plain,
            drawing: Drawing::BuiltIn,
            screen,
            focus: Focus::Now,
            landing: Landing::Now,
            menu: Menu::CLOSED,
            position: None,
            heading: None,
            access: LocationAccess::Allowed,
            trail: Vec::new(),
            now: TripTime::on_date_at(day.zone(), day.date(), time),
            progress: LegProgress::default(),
            halt: None,
            extra: Vec::new(),
            clock: None,
            origin: None,
            reports: None,
            refused: None,
        }
    }

    fn day_stages(day: &Day) -> Vec<Stage> {
        let legs = day.legs();
        let starts = legs.iter().flat_map(|leg| {
            let time = leg.time(End::Start).later_by(Self::AFTER_START);
            [
                Self::timed(day, Screen::Map, time, Situation::LegStart),
                Self::timed(day, Screen::Today, time, Situation::LegStart),
            ]
        });
        let boarding = legs
            .iter()
            .filter(|leg| leg.mode() != Mode::Walk)
            .flat_map(|leg| {
                let time = leg.time(End::Start).later_by(Self::BEFORE_RIDE);
                [
                    Self::timed(day, Screen::Map, time, Situation::WaitingForRide),
                    Self::timed(day, Screen::Today, time, Situation::WaitingForRide),
                ]
            });
        let longest = legs
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() == Mode::Walk)
            .max_by_key(|(_, leg)| leg.length());
        let walking = longest.into_iter().flat_map(|(position, leg)| {
            let on_time =
                LegProgress::confirmed(day.number(), LegIndex::new(position), Minutes::ZERO);
            let halfway = leg.line().length() * Share::HALF;
            let middle = leg.time(End::Start).later_by(leg.planned_after(halfway));
            let late = leg.time(End::Finish).later_by(Self::LATE);
            let on_time = leg.line().point_along(halfway).map_or(on_time, |seen| {
                on_time
                    .placed(
                        halfway,
                        TripTime::on_date_at(
                            day.zone(),
                            day.date(),
                            late.later_by(-Self::SEEN_BEFORE),
                        ),
                    )
                    .seen_at(seen)
            });
            [
                Self::timed(day, Screen::Map, middle, Situation::WalkingWithGps).walking_on(leg),
                Self::timed(day, Screen::Map, middle, Situation::OffRoute)
                    .off_route_from(&leg.point(End::Start)),
                Self::timed(day, Screen::Map, late, Situation::Late)
                    .walking_on(leg)
                    .confirmed(on_time),
                Self::timed(day, Screen::Today, late, Situation::Late)
                    .walking_on(leg)
                    .confirmed(on_time),
                Self::timed(day, Screen::Map, late, Situation::BrowsingWhileLate)
                    .walking_on(leg)
                    .confirmed(on_time)
                    .browsing_at(day, Self::middle_of(day))
                    .requiring(&Self::LIVE_LATE),
            ]
        });
        let ends = legs
            .first()
            .zip(legs.last())
            .into_iter()
            .flat_map(|(first, last)| {
                [
                    Self::timed(
                        day,
                        Screen::Map,
                        first.time(End::Start),
                        Situation::WeekEarly,
                    )
                    .ticked_before(Self::WEEK),
                    Self::timed(
                        day,
                        Screen::Today,
                        first.time(End::Start),
                        Situation::WeekEarly,
                    )
                    .ticked_before(Self::WEEK),
                    Self::timed(
                        day,
                        Screen::Map,
                        last.time(End::Finish).later_by(Self::LATE),
                        Situation::DayDone,
                    ),
                ]
            });
        starts.chain(boarding).chain(walking).chain(ends).collect()
    }
}

#[cfg(test)]
impl Stage {
    pub fn marks_its_focus(&self, page: &str) -> bool {
        self.focus.spot().is_some_and(|spot| {
            let named = if self.screen == Screen::Map {
                format!("BROWSING · Day {} · step {}<", spot.day(), spot.step())
            } else {
                format!("class=\"day-title\">Day {} ·", spot.day())
            };
            page.contains(&named)
                && (self.screen != Screen::Today
                    || page
                        .split(&format!("id=\"{}\"", spot.step().anchor()))
                        .nth(1)
                        .and_then(|rest| rest.split('>').next())
                        .is_some_and(|tag| tag.contains("focused")))
        })
    }

    pub fn on_schedule(itinerary: &Itinerary, walked_for: Minutes) -> Option<Stage> {
        WalkScene::all(itinerary)
            .iter()
            .find(|scene| {
                scene.day().legs().first().is_some_and(|first| {
                    first.mode() == Mode::Walk
                        && first.time(End::Start).later_by(walked_for) < scene.time()
                })
            })
            .map(|scene| Self::walking(scene, Screen::Map, Situation::WalkingWithGps))
    }
}

impl Staged for Stage {
    fn screen(&self) -> &Screen {
        &self.screen
    }

    fn map_level(&self) -> MapLevel {
        MapLevel::default()
    }

    fn map_mode(&self) -> MapMode {
        match self.keyed {
            Keyed::Exploring => MapMode::Explore,
            Keyed::Plain | Keyed::Tapped => MapMode::Guide,
        }
    }

    fn key_shown(&self) -> Vec<KeyEntry> {
        match self.keyed {
            Keyed::Plain | Keyed::Exploring => Vec::new(),
            Keyed::Tapped => Self::KEY.to_vec(),
        }
    }

    fn picked(&self) -> Option<PickedMark> {
        match self.keyed {
            Keyed::Plain | Keyed::Exploring => None,
            Keyed::Tapped => Some(PickedMark::new(KeyEntry::PoiKonbini, None)),
        }
    }

    fn focus(&self) -> Focus {
        self.focus
    }

    fn landing(&self) -> Landing {
        self.landing
    }

    fn menu(&self) -> Menu {
        self.menu
    }

    fn viewport(&self) -> Viewport {
        Handset::viewport()
    }

    fn position(&self) -> Option<Coordinate> {
        self.position
    }

    fn spread(&self) -> Option<Distance> {
        self.position.map(|_| Self::SPREAD)
    }

    fn heading(&self) -> Option<Bearing> {
        self.heading
    }

    fn access(&self) -> LocationAccess {
        self.access
    }

    fn trail(&self) -> &[Coordinate] {
        &self.trail
    }

    fn halt(&self) -> Option<Halt> {
        self.halt
    }

    fn progress(&self) -> LegProgress {
        self.progress
    }

    fn now(&self) -> TripTime {
        self.now
    }

    fn refusal(&self) -> Option<TripRefusal> {
        self.refused.as_ref().and_then(|picked| picked.trip().err())
    }

    fn clock(&self) -> WallClock {
        WallClock::Stopped(self.clock.unwrap_or(self.now))
    }

    fn wiki(&self) -> Arc<Wiki> {
        self.drawing.wiki()
    }

    fn starting_history(&self) -> History {
        self.origin
            .iter()
            .fold(History::default(), |history, origin| {
                history.left(Mark::new(
                    *origin,
                    Visit::default(),
                    StepFolds::default(),
                    ScrollTop::default(),
                ))
            })
    }

    fn reports(&self) -> Option<ReportStore> {
        self.reports.clone()
    }
}

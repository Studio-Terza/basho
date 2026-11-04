mod map_mark;

use std::sync::LazyLock;

use guide::Trip;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::{Coordinate, Distance};
use guide::guidance::{Guidance, LegProgress};
use guide::itinerary::day::goal::Purpose;
use guide::itinerary::{Caption, Day};
use guide::leg::{LegIndex, Mode};
use guide::reading::Reading;
use guide::times::PlanTime;
use guide::words::Spelling;

use map_plan::{KeyEntry, MapLevel, MapMode, MapPlan, MapSent, MapSight, PinchWay};

const ASTRAY: Distance = Distance::new(1500.0);
const ALONG: Distance = Distance::new(60.0);
const TRIP: &str = include_str!("../../../../data/trips/sample.toml");

static BUNDLED: LazyLock<Trip> = LazyLock::new(|| TRIP.parse().expect("the trip loads"));

struct Walking<'trip> {
    day: &'trip Day,
    position: usize,
}

impl<'trip> Walking<'trip> {
    fn all() -> Vec<Walking<'static>> {
        BUNDLED
            .itinerary()
            .days()
            .iter()
            .flat_map(|day| {
                let legs = day.legs().iter().enumerate();
                legs.filter(|(_, leg)| leg.mode() == Mode::Walk)
                    .map(move |(position, _)| Walking { day, position })
            })
            .collect()
    }

    fn guidance(&self, here: Option<Coordinate>) -> Guidance<'trip> {
        let leg = &self.day.legs()[self.position];
        let time = leg.time(End::Start);
        let now = TripTime::on_date_at(self.day.zone(), self.day.date(), time);
        Guidance::new(self.day, LegProgress::default(), now, here)
    }

    fn sight<'view>(&self, guidance: &'view Guidance<'view>) -> MapSight<'view> {
        MapSight::browsing(guidance, LegIndex::new(self.position))
    }

    fn plan<'view>(
        guidance: &'view Guidance<'view>,
        sight: MapSight<'view>,
        level: MapLevel,
    ) -> MapPlan<'view> {
        MapPlan::new(BUNDLED.overview(), guidance, sight, level).expect("a day has legs")
    }
}

#[test]
fn every_walking_leg_is_drawn_as_the_current_line() {
    let missing: Vec<String> = Walking::all()
        .iter()
        .filter(|walking| {
            let guidance = walking.guidance(None);
            let route = Walking::plan(&guidance, walking.sight(&guidance), MapLevel::Leg).route();
            serde_json::to_string(&route)
                .expect("a route serializes")
                .matches(r#""state":"current""#)
                .count()
                == 0
        })
        .map(|walking| format!("day {} leg {}", walking.day.number(), walking.position))
        .collect();
    assert_eq!(missing, Vec::<String>::new());
}

#[test]
fn a_walker_off_the_route_is_inside_the_frame() {
    let walking = &Walking::all()[0];
    let start = walking.day.legs()[walking.position].point(End::Start);
    let here = start.offset_by(Distance::default(), ASTRAY);
    let guidance = walking.guidance(Some(here));
    let bounds = Walking::plan(&guidance, MapSight::live(&guidance), MapLevel::Leg).bounds();
    let corners: [[f64; 2]; 2] =
        serde_json::from_value(serde_json::to_value(bounds).expect("bounds serialize"))
            .expect("two corners");
    let lat = here.lat().angle().degrees();
    assert!((corners[0][1]..=corners[1][1]).contains(&lat));
}

#[test]
fn wider_levels_frame_more_of_the_day() {
    let walking = &Walking::all()[0];
    let guidance = walking.guidance(None);
    let span = |level| {
        let bounds = Walking::plan(&guidance, walking.sight(&guidance), level).bounds();
        let corners: [[f64; 2]; 2] =
            serde_json::from_value(serde_json::to_value(bounds).expect("bounds serialize"))
                .expect("two corners");
        corners[1][0] - corners[0][0]
    };
    assert!(span(MapLevel::Leg) <= span(MapLevel::Goal));
    assert!(span(MapLevel::Goal) <= span(MapLevel::Day));
}

#[test]
fn a_position_fix_sends_only_the_walker() {
    let walking = &Walking::all()[0];
    let place = Spelling::new("Inn".to_owned());
    let leg = &walking.day.legs()[walking.position];
    let first = walking.guidance(leg.line().point_along(ALONG));
    let moved = walking.guidance(leg.line().point_along(ALONG + ALONG));
    let mut sent = MapSent::default();
    let whole = sent
        .change_for(
            &Walking::plan(&first, MapSight::live(&first), MapLevel::Leg),
            &place,
            &[],
            None,
        )
        .expect("the first scene is sent");
    let fix = sent
        .change_for(
            &Walking::plan(&moved, MapSight::live(&moved), MapLevel::Leg),
            &place,
            &[],
            None,
        )
        .expect("the walker moved");
    assert!(whole.route().is_some());
    assert!(whole.marks().is_some());
    assert!(fix.route().is_none());
    assert!(fix.marks().is_none());
    assert!(fix.walker().is_some());
}

#[test]
fn an_unchanged_scene_sends_nothing_and_a_mode_goes_once() {
    let walking = &Walking::all()[0];
    let place = Spelling::new("Inn".to_owned());
    let guidance = walking.guidance(None);
    let shown = Walking::plan(&guidance, walking.sight(&guidance), MapLevel::Leg);
    let mut sent = MapSent::default();
    assert!(sent.change_for(&shown, &place, &[], None).is_some());
    assert_eq!(sent.change_for(&shown, &place, &[], None), None);
    assert!(sent.mode_update(MapMode::Explore).is_some());
    assert_eq!(sent.mode_update(MapMode::Explore), None);
}

#[test]
fn pinching_walks_the_levels_and_stops_at_the_ends() {
    let together = MapLevel::Leg.pinched(PinchWay::Together);
    assert_eq!(together, MapLevel::Goal);
    assert_eq!(together.pinched(PinchWay::Together), MapLevel::Day);
    assert_eq!(MapLevel::Day.pinched(PinchWay::Together), MapLevel::Day);
    assert_eq!(MapLevel::Goal.pinched(PinchWay::Apart), MapLevel::Leg);
    assert_eq!(MapLevel::Leg.pinched(PinchWay::Apart), MapLevel::Leg);
}

#[test]
fn the_key_reads_what_the_engine_names_and_ignores_the_rest() {
    let heard: Vec<KeyEntry> =
        serde_json::from_str(r#"["road_major","goal","no_such_thing"]"#).expect("a key list");
    let words: Vec<String> = heard
        .iter()
        .map(|entry| entry.meaning().to_string())
        .collect();
    assert_eq!(heard[2], KeyEntry::Unlisted);
    assert_eq!(words[0], "Major road");
    assert_eq!(words[1], "The place at the end of this step");
}

#[test]
fn a_loop_shows_numbered_stops_in_order_on_the_goal_map_and_nowhere_else() {
    let place = Spelling::new("Town".to_owned());
    let looped = Walking::all()
        .into_iter()
        .find(|walking| {
            walking
                .day
                .goal_holding(LegIndex::new(walking.position))
                .map_or(0, |goal| goal.purpose().stops().len())
                > 0
        })
        .expect("the trip holds a loop");
    let guidance = looped.guidance(None);
    let marks = |level| {
        serde_json::to_string(
            &Walking::plan(&guidance, looped.sight(&guidance), level).marks(&place),
        )
        .expect("marks serialize")
    };
    let goal = marks(MapLevel::Goal);
    let labels: Vec<String> = looped
        .day
        .goal_holding(LegIndex::new(looped.position))
        .into_iter()
        .flat_map(|goal| goal.purpose().stops())
        .map(|stop| {
            let planned = AsRef::<PlanTime>::as_ref(&stop);
            format!(r#""label":"{} · {planned} "#, stop.number())
        })
        .collect();
    let places: Vec<usize> = labels
        .iter()
        .filter_map(|label| goal.find(label.as_str()))
        .collect();
    assert_eq!(places.len(), labels.len());
    assert!(places.is_sorted());
    assert_eq!(goal.matches("loop_stop").count(), labels.len());
    assert_eq!(marks(MapLevel::Leg).matches("loop_stop").count(), 0);
    assert_eq!(marks(MapLevel::Day).matches("loop_stop").count(), 0);
}

#[test]
fn an_unverified_inn_goal_carries_the_mark_in_its_map_label() {
    let place = Spelling::new("Town".to_owned());
    let labels: Vec<bool> = Walking::all()
        .into_iter()
        .filter(|walking| walking.position == 0)
        .flat_map(|walking| {
            let guidance = walking.guidance(None);
            let marks = serde_json::to_string(
                &Walking::plan(&guidance, walking.sight(&guidance), MapLevel::Day).marks(&place),
            )
            .expect("marks serialize");
            walking
                .day
                .goals()
                .iter()
                .map(|goal| {
                    let name = goal[Caption::Name].text(Reading::Spoken);
                    marks.contains(&format!("\"{name} ?\""))
                })
                .collect::<Vec<bool>>()
        })
        .collect();
    let inns: Vec<bool> = Walking::all()
        .into_iter()
        .filter(|walking| walking.position == 0)
        .flat_map(|walking| walking.day.goals().iter())
        .map(|goal| matches!(goal.purpose(), Purpose::Inn))
        .collect();
    assert!(inns.contains(&true));
    assert!(inns.contains(&false));
    assert_eq!(labels, inns);
}

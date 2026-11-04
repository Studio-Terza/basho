use guide::classed::Classed;
use guide::words::Label;
use serde::Deserialize;

use crate::map_mark::MapMark;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "kebab-case")]
pub enum KeyEntry {
    RouteCurrent,
    RouteLater,
    RouteDone,
    RouteRide,
    RouteNoPath,
    Trail,
    RoadHighway,
    RoadMajor,
    RoadMinor,
    RoadStreet,
    RoadLane,
    RoadUnpaved,
    RoadTrail,
    Bridge,
    Tunnel,
    Rail,
    Ferry,
    Water,
    Forest,
    Building,
    PoiKonbini,
    PoiFood,
    PoiShop,
    PoiService,
    PoiToilet,
    PoiLodging,
    PoiWorship,
    PoiSight,
    PoiStation,
    PoiPeak,
    Food,
    Shop,
    Sight,
    Service,
    Landmark,
    Catch,
    Star,
    Peak,
    DayStart,
    LegEnd,
    Goal,
    LoopStop,
    Night,
    You,
    YouEstimate,
    YouLastFix,
    NoPath,
    #[serde(other)]
    Unlisted,
}

impl Classed for KeyEntry {}

impl KeyEntry {
    #[must_use]
    pub fn mark(self) -> Option<MapMark> {
        match self {
            KeyEntry::Food => Some(MapMark::Food),
            KeyEntry::Shop => Some(MapMark::Shop),
            KeyEntry::Sight => Some(MapMark::Sight),
            KeyEntry::Service => Some(MapMark::Service),
            KeyEntry::Landmark => Some(MapMark::Landmark),
            KeyEntry::Catch => Some(MapMark::Catch),
            KeyEntry::Star => Some(MapMark::Star),
            KeyEntry::Peak => Some(MapMark::Peak),
            KeyEntry::DayStart => Some(MapMark::DayStart),
            KeyEntry::LegEnd => Some(MapMark::LegEnd),
            KeyEntry::Goal => Some(MapMark::Goal),
            KeyEntry::LoopStop => Some(MapMark::LoopStop),
            KeyEntry::Night => Some(MapMark::Night),
            KeyEntry::You => Some(MapMark::You),
            KeyEntry::YouEstimate => Some(MapMark::YouEstimate),
            KeyEntry::YouLastFix => Some(MapMark::YouLastFix),
            KeyEntry::NoPath => Some(MapMark::NoPath),
            _ => None,
        }
    }

    #[must_use]
    pub fn meaning(self) -> Label {
        match self {
            KeyEntry::RouteCurrent => Label::new("This leg"),
            KeyEntry::RouteLater => Label::new("Legs to come"),
            KeyEntry::RouteDone => Label::new("Legs walked"),
            KeyEntry::RouteRide => Label::new("Train, bus or boat"),
            KeyEntry::RouteNoPath => Label::new("No mapped path"),
            KeyEntry::Trail => Label::new("Where you have been"),
            KeyEntry::RoadHighway => Label::new("Highway"),
            KeyEntry::RoadMajor => Label::new("Major road"),
            KeyEntry::RoadMinor => Label::new("Minor road"),
            KeyEntry::RoadStreet => Label::new("Street"),
            KeyEntry::RoadLane => Label::new("Lane or paved path"),
            KeyEntry::RoadUnpaved => Label::new("Unpaved track"),
            KeyEntry::RoadTrail => Label::new("Trail"),
            KeyEntry::Bridge => Label::new("Bridge"),
            KeyEntry::Tunnel => Label::new("Tunnel"),
            KeyEntry::Rail => Label::new("Railway"),
            KeyEntry::Ferry => Label::new("Ferry"),
            KeyEntry::Water => Label::new("Water"),
            KeyEntry::Forest => Label::new("Forest"),
            KeyEntry::Building => Label::new("Building"),
            KeyEntry::PoiKonbini => Label::new("Konbini"),
            KeyEntry::PoiFood => Label::new("Food, not on the plan"),
            KeyEntry::PoiShop => Label::new("Shop, not on the plan"),
            KeyEntry::PoiService => Label::new("Bank, post, clinic, bath"),
            KeyEntry::PoiToilet => Label::new("Toilet or water"),
            KeyEntry::PoiLodging => Label::new("Lodging"),
            KeyEntry::PoiWorship => Label::new("Shrine or temple"),
            KeyEntry::PoiSight => Label::new("Sight, not on the plan"),
            KeyEntry::PoiStation => Label::new("Station"),
            KeyEntry::PoiPeak => Label::new("Mountain"),
            KeyEntry::Unlisted
            | KeyEntry::Food
            | KeyEntry::Shop
            | KeyEntry::Sight
            | KeyEntry::Service
            | KeyEntry::Landmark
            | KeyEntry::Catch
            | KeyEntry::Star
            | KeyEntry::Peak
            | KeyEntry::DayStart
            | KeyEntry::LegEnd
            | KeyEntry::Goal
            | KeyEntry::LoopStop
            | KeyEntry::Night
            | KeyEntry::You
            | KeyEntry::YouEstimate
            | KeyEntry::YouLastFix
            | KeyEntry::NoPath => self.mark().map_or(Label::new(""), MapMark::meaning),
        }
    }
}

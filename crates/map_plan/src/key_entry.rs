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
        self.mark().map_or_else(
            || {
                Label::new(match self {
                    KeyEntry::RouteCurrent => "This leg",
                    KeyEntry::RouteLater => "Legs to come",
                    KeyEntry::RouteDone => "Legs walked",
                    KeyEntry::RouteRide => "Train, bus or boat",
                    KeyEntry::RouteNoPath => "No mapped path",
                    KeyEntry::Trail => "Where you have been",
                    KeyEntry::RoadHighway => "Highway",
                    KeyEntry::RoadMajor => "Major road",
                    KeyEntry::RoadMinor => "Minor road",
                    KeyEntry::RoadStreet => "Street",
                    KeyEntry::RoadLane => "Lane or paved path",
                    KeyEntry::RoadUnpaved => "Unpaved track",
                    KeyEntry::RoadTrail => "Trail",
                    KeyEntry::Bridge => "Bridge",
                    KeyEntry::Tunnel => "Tunnel",
                    KeyEntry::Rail => "Railway",
                    KeyEntry::Ferry => "Ferry",
                    KeyEntry::Water => "Water",
                    KeyEntry::Forest => "Forest",
                    KeyEntry::Building => "Building",
                    KeyEntry::PoiKonbini => "Konbini",
                    KeyEntry::PoiFood => "Food, not on the plan",
                    KeyEntry::PoiShop => "Shop, not on the plan",
                    KeyEntry::PoiService => "Bank, post, clinic, bath",
                    KeyEntry::PoiToilet => "Toilet or water",
                    KeyEntry::PoiLodging => "Lodging",
                    KeyEntry::PoiWorship => "Shrine or temple",
                    KeyEntry::PoiSight => "Sight, not on the plan",
                    KeyEntry::PoiStation => "Station",
                    KeyEntry::PoiPeak => "Mountain",
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
                    | KeyEntry::NoPath => "",
                })
            },
            MapMark::meaning,
        )
    }
}

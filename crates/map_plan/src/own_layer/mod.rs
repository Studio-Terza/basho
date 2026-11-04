mod bits;
mod geo_heading;

use std::cmp::Ordering;

use guide::classed::Classed;
use guide::geo::{Bearing, Coordinate};
use guide::leg::{LegIndex, Mode};
use serde::Serialize;

use crate::map_mark::{MapMark, Strength};
use crate::map_text::MapText;
pub(crate) use bits::GeoPoint;
use geo_heading::GeoHeading;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename = "FeatureCollection")]
pub struct OwnLayer<Look> {
    features: Vec<OwnFeature<Look>>,
}

impl<Look> FromIterator<OwnFeature<Look>> for OwnLayer<Look> {
    fn from_iter<Features: IntoIterator<Item = OwnFeature<Look>>>(features: Features) -> Self {
        OwnLayer {
            features: features.into_iter().collect(),
        }
    }
}

impl<Look> OwnLayer<Look> {
    #[must_use]
    pub fn features(&self) -> &[OwnFeature<Look>] {
        &self.features
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename = "Feature")]
pub struct OwnFeature<Look> {
    properties: Look,
    geometry: OwnGeometry,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", content = "coordinates")]
enum OwnGeometry {
    Point(GeoPoint),
    LineString(Vec<GeoPoint>),
}

impl<Look> OwnFeature<Look> {
    #[must_use]
    pub fn spot(properties: Look, at: Coordinate) -> OwnFeature<Look> {
        OwnFeature {
            properties,
            geometry: OwnGeometry::Point(GeoPoint::from(at)),
        }
    }

    #[must_use]
    pub fn line(properties: Look, points: Vec<Coordinate>) -> OwnFeature<Look> {
        OwnFeature {
            properties,
            geometry: OwnGeometry::LineString(points.into_iter().map(GeoPoint::from).collect()),
        }
    }

    #[must_use]
    pub fn look(&self) -> &Look {
        &self.properties
    }
}

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Travel {
    Walk,
    Ride,
    NoPath,
}

impl Travel {
    #[must_use]
    pub fn of_mode(mode: Mode) -> Travel {
        match mode {
            Mode::Walk => Travel::Walk,
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Travel::Ride,
        }
    }
}

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LegState {
    Done,
    Current,
    Later,
}

impl LegState {
    #[must_use]
    pub fn of_progress(index: LegIndex, current: Option<LegIndex>) -> LegState {
        match current {
            Some(current) => Self::against(index.cmp(&current)),
            None => LegState::Done,
        }
    }

    fn against(order: Ordering) -> LegState {
        match order {
            Ordering::Equal => LegState::Current,
            Ordering::Greater => LegState::Later,
            Ordering::Less => LegState::Done,
        }
    }
}

impl Classed for LegState {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct RouteLook {
    travel: Travel,
    state: LegState,
}

impl RouteLook {
    #[must_use]
    pub fn new(travel: Travel, state: LegState) -> RouteLook {
        RouteLook { travel, state }
    }

    #[must_use]
    pub fn state(self) -> LegState {
        self.state
    }

    #[must_use]
    pub fn travel(self) -> Travel {
        self.travel
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MarkLook {
    mark: MapMark,
    strength: Strength,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<MapText>,
}

impl MarkLook {
    #[must_use]
    pub fn plain(mark: MapMark) -> MarkLook {
        MarkLook {
            mark,
            strength: mark.strength(),
            label: None,
        }
    }

    #[must_use]
    pub fn named(mark: MapMark, label: MapText) -> MarkLook {
        MarkLook {
            label: Some(label),
            ..Self::plain(mark)
        }
    }

    #[must_use]
    pub fn mark(&self) -> MapMark {
        self.mark
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalkerPiece {
    You,
    Cone,
    Trail,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct WalkerPart {
    part: WalkerPiece,
    #[serde(skip_serializing_if = "Option::is_none")]
    mark: Option<MapMark>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<GeoHeading>,
}

impl WalkerPart {
    #[must_use]
    pub fn you(mark: MapMark) -> WalkerPart {
        WalkerPart {
            part: WalkerPiece::You,
            mark: Some(mark),
            heading: None,
        }
    }

    #[must_use]
    pub fn cone(facing: Bearing) -> WalkerPart {
        WalkerPart {
            part: WalkerPiece::Cone,
            mark: None,
            heading: Some(GeoHeading::from(facing)),
        }
    }

    #[must_use]
    pub fn trail() -> WalkerPart {
        WalkerPart {
            part: WalkerPiece::Trail,
            mark: None,
            heading: None,
        }
    }

    #[must_use]
    pub fn piece(self) -> WalkerPiece {
        self.part
    }
}

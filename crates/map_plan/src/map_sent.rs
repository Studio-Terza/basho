use guide::geo::{Bearing, Coordinate};
use guide::words::Spelling;
use serde::Serialize;

use crate::bits::MapBounds;
use crate::own_layer::{MarkLook, OwnLayer, RouteLook, WalkerPart};
use crate::{MapMode, MapPlan};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct MapUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    route: Option<OwnLayer<RouteLook>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marks: Option<OwnLayer<MarkLook>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    walker: Option<OwnLayer<WalkerPart>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounds: Option<MapBounds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<MapMode>,
}

impl MapUpdate {
    #[must_use]
    pub fn route(&self) -> Option<&OwnLayer<RouteLook>> {
        self.route.as_ref()
    }

    #[must_use]
    pub fn marks(&self) -> Option<&OwnLayer<MarkLook>> {
        self.marks.as_ref()
    }

    #[must_use]
    pub fn walker(&self) -> Option<&OwnLayer<WalkerPart>> {
        self.walker.as_ref()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MapSent {
    latest: MapUpdate,
    mode: Option<MapMode>,
}

impl MapSent {
    pub fn change_for(
        &mut self,
        plan: &MapPlan<'_>,
        place: &Spelling,
        walked: &[Coordinate],
        heading: Option<Bearing>,
    ) -> Option<MapUpdate> {
        let latest = MapUpdate {
            route: Some(plan.route()),
            marks: Some(plan.marks(place)),
            walker: Some(plan.walker(walked, heading)),
            bounds: plan.bounds(),
            mode: None,
        };
        let change = MapUpdate {
            route: latest
                .route
                .clone()
                .filter(|_| latest.route != self.latest.route),
            marks: latest
                .marks
                .clone()
                .filter(|_| latest.marks != self.latest.marks),
            walker: latest
                .walker
                .clone()
                .filter(|_| latest.walker != self.latest.walker),
            bounds: latest
                .bounds
                .filter(|_| latest.bounds != self.latest.bounds),
            mode: None,
        };
        self.latest = latest;
        (change != MapUpdate::default()).then_some(change)
    }

    pub fn mode_update(&mut self, mode: MapMode) -> Option<MapUpdate> {
        let before = self.mode.replace(mode);
        (before != Some(mode)).then(|| MapUpdate {
            mode: Some(mode),
            ..MapUpdate::default()
        })
    }
}

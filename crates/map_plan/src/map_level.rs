use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MapLevel {
    #[default]
    Leg,
    Goal,
    Day,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PinchWay {
    Together,
    Apart,
}

impl MapLevel {
    #[must_use]
    pub fn pinched(self, way: PinchWay) -> MapLevel {
        match (self, way) {
            (MapLevel::Leg | MapLevel::Goal, PinchWay::Apart) => MapLevel::Leg,
            (MapLevel::Day, PinchWay::Apart) | (MapLevel::Leg, PinchWay::Together) => {
                MapLevel::Goal
            }
            (MapLevel::Goal | MapLevel::Day, PinchWay::Together) => MapLevel::Day,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MapMode {
    #[default]
    Guide,
    Explore,
}

impl MapMode {
    #[must_use]
    pub fn switched(self) -> MapMode {
        match self {
            MapMode::Guide => MapMode::Explore,
            MapMode::Explore => MapMode::Guide,
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
    const LEVELS: [MapLevel; 3] = [MapLevel::Leg, MapLevel::Goal, MapLevel::Day];

    #[must_use]
    pub fn pinched(self, way: PinchWay) -> MapLevel {
        match way {
            PinchWay::Apart => Self::LEVELS
                .into_iter()
                .rev()
                .find(|level| *level < self)
                .unwrap_or(self),
            PinchWay::Together => Self::LEVELS
                .into_iter()
                .find(|level| *level > self)
                .unwrap_or(self),
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

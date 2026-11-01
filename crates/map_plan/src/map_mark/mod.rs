mod look;
mod mark_at;
mod shape;
mod strength;
mod tone;

use guide::guidance::PositionSource;
use guide::leg::StopKind;
use guide::words::Label;
use serde::Serialize;

use layout::measure::MarkSize;
pub use look::Look;
pub use mark_at::MarkAt;
use shape::Shape;
pub use strength::Strength;
use tone::Tone;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MapMark {
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
}

impl MapMark {
    pub const ALL: [MapMark; 17] = [
        MapMark::Star,
        MapMark::Peak,
        MapMark::Goal,
        MapMark::LoopStop,
        MapMark::LegEnd,
        MapMark::DayStart,
        MapMark::Night,
        MapMark::You,
        MapMark::YouEstimate,
        MapMark::YouLastFix,
        MapMark::Food,
        MapMark::Shop,
        MapMark::Sight,
        MapMark::Service,
        MapMark::Landmark,
        MapMark::Catch,
        MapMark::NoPath,
    ];

    #[must_use]
    pub fn stop(kind: StopKind) -> MapMark {
        match kind {
            StopKind::Food => MapMark::Food,
            StopKind::Shop => MapMark::Shop,
            StopKind::Sight => MapMark::Sight,
            StopKind::Service => MapMark::Service,
            StopKind::Landmark => MapMark::Landmark,
        }
    }

    #[must_use]
    pub fn you(source: PositionSource) -> MapMark {
        match source {
            PositionSource::Gps => MapMark::You,
            PositionSource::Estimate => MapMark::YouEstimate,
            PositionSource::GpsLost { .. } => MapMark::YouLastFix,
        }
    }

    #[must_use]
    pub fn look(self) -> Look {
        match self {
            MapMark::Food => Look::new(Shape::Square, Tone::Solid, MarkSize::Small),
            MapMark::Shop => Look::new(Shape::Diamond, Tone::Hollow, MarkSize::Small),
            MapMark::Sight => Look::new(Shape::Diamond, Tone::Solid, MarkSize::Small),
            MapMark::Service => Look::new(Shape::Cross, Tone::Stroke, MarkSize::Small),
            MapMark::Landmark => Look::new(Shape::Bar, Tone::Stroke, MarkSize::Small),
            MapMark::Catch => Look::new(Shape::Square, Tone::Hollow, MarkSize::Small),
            MapMark::Star => Look::new(Shape::Star, Tone::Solid, MarkSize::Large),
            MapMark::Peak => Look::new(Shape::Triangle, Tone::Solid, MarkSize::Large),
            MapMark::DayStart => Look::new(Shape::Flag, Tone::Solid, MarkSize::Large),
            MapMark::LegEnd => Look::new(Shape::Tick, Tone::Stroke, MarkSize::Small),
            MapMark::Goal => Look::new(Shape::Target, Tone::Solid, MarkSize::Large),
            MapMark::LoopStop => Look::new(Shape::Dot, Tone::Solid, MarkSize::Large),
            MapMark::Night => Look::new(Shape::House, Tone::Solid, MarkSize::Large),
            MapMark::You => Look::new(Shape::Ringed, Tone::Solid, MarkSize::Large),
            MapMark::YouEstimate => Look::new(Shape::Ringed, Tone::Hollow, MarkSize::Large),
            MapMark::YouLastFix => Look::new(Shape::Ex, Tone::Stroke, MarkSize::Large),
            MapMark::NoPath => Look::new(Shape::Triangle, Tone::Hollow, MarkSize::Small),
        }
    }

    #[must_use]
    pub fn meaning(self) -> Label {
        Label::new(match self {
            MapMark::Food => "Food",
            MapMark::Shop => "Shop",
            MapMark::Sight => "Sight",
            MapMark::Service => "Toilet, water",
            MapMark::Landmark => "Landmark",
            MapMark::Catch => "Something to walk toward",
            MapMark::Star => "A starred point of the trip",
            MapMark::Peak => "Mountain, with its height in metres",
            MapMark::DayStart => "Where the day starts",
            MapMark::LegEnd => "Where one leg ends and the next begins",
            MapMark::Goal => "The place at the end of this step",
            MapMark::LoopStop => "A sight of the loop, numbered in the order to see it",
            MapMark::Night => "Tonight's lodging",
            MapMark::You => "You, from GPS",
            MapMark::YouEstimate => "You, estimated from the timetable",
            MapMark::YouLastFix => "You at the last GPS fix, now old",
            MapMark::NoPath => "No mapped path here",
        })
    }

    #[must_use]
    pub fn strength(self) -> Strength {
        match self {
            MapMark::Food
            | MapMark::Shop
            | MapMark::Sight
            | MapMark::Service
            | MapMark::Landmark
            | MapMark::LegEnd => Strength::Weak,
            MapMark::Star
            | MapMark::Peak
            | MapMark::Catch
            | MapMark::NoPath
            | MapMark::DayStart
            | MapMark::Goal
            | MapMark::LoopStop
            | MapMark::Night
            | MapMark::You
            | MapMark::YouEstimate
            | MapMark::YouLastFix => Strength::Strong,
        }
    }
}

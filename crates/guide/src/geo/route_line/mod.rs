pub mod route_fix;

use crate::geo::coordinate::Coordinate;
use crate::geo::distance::Distance;
use crate::geo::share::Share;

pub use route_fix::RouteFix;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteLine {
    points: Vec<Coordinate>,
    walked: Vec<Distance>,
}

impl RouteLine {
    pub const SKETCH_SEGMENT: Distance = Distance::new(1000.0);
    pub const SEGMENT_ENDS_LENGTH: usize = 2;

    #[must_use]
    pub fn new(points: Vec<Coordinate>) -> RouteLine {
        let walked = points
            .first()
            .map(|_| Distance::default())
            .into_iter()
            .chain(points.windows(Self::SEGMENT_ENDS_LENGTH).scan(
                Distance::default(),
                |walked, pair| {
                    *walked = *walked + pair[0].distance_to(pair[1]);
                    Some(*walked)
                },
            ))
            .collect();
        RouteLine { points, walked }
    }

    #[must_use]
    pub fn length(&self) -> Distance {
        self.walked.last().copied().unwrap_or_default()
    }

    #[must_use]
    pub fn is_sketch(&self) -> bool {
        self.segment_lengths()
            .any(|length| length > Self::SKETCH_SEGMENT)
    }

    #[must_use]
    pub fn longest_segment(&self) -> Option<Distance> {
        self.segment_lengths().max()
    }

    fn segment_lengths(&self) -> impl Iterator<Item = Distance> + '_ {
        self.walked
            .windows(Self::SEGMENT_ENDS_LENGTH)
            .map(|pair| pair[1] - pair[0])
    }

    #[must_use]
    pub fn points_beyond(&self, along: Distance) -> Vec<Coordinate> {
        self.points
            .iter()
            .zip(&self.walked)
            .filter(|(_, walked)| **walked >= along)
            .map(|(point, _)| *point)
            .collect()
    }

    #[must_use]
    pub fn fix(&self, position: Coordinate) -> Option<RouteFix> {
        self.fixes(position)
            .into_iter()
            .min_by_key(|fix| fix.off_route())
    }

    #[must_use]
    pub fn fixes(&self, position: Coordinate) -> Vec<RouteFix> {
        self.points
            .windows(Self::SEGMENT_ENDS_LENGTH)
            .zip(self.walked.windows(Self::SEGMENT_ENDS_LENGTH))
            .map(|(pair, walked)| {
                RouteFix::on_measured_segment([pair[0], pair[1]], [walked[0], walked[1]], position)
            })
            .chain(
                self.lone_point()
                    .map(|only| RouteFix::on_segment(only, only, Distance::default(), position)),
            )
            .collect()
    }

    #[must_use]
    pub fn point_along(&self, distance: Distance) -> Option<Coordinate> {
        self.points
            .windows(Self::SEGMENT_ENDS_LENGTH)
            .zip(self.walked.windows(Self::SEGMENT_ENDS_LENGTH))
            .find(|(_, walked)| walked[1].meters() >= distance.meters())
            .map(|(pair, walked)| {
                pair[0].toward(
                    pair[1],
                    Share::ratio(distance - walked[0], walked[1] - walked[0]),
                )
            })
            .or_else(|| self.points.last().copied())
    }

    fn lone_point(&self) -> Option<Coordinate> {
        match self.points.as_slice() {
            [only] => Some(*only),
            _ => None,
        }
    }
}

impl RouteLine {
    #[must_use]
    pub fn points(&self) -> &[Coordinate] {
        &self.points
    }
}

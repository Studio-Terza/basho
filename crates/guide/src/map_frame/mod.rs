mod axis;
mod pixels;
mod plane_point;
mod scale;
mod svg_path;
mod tenths;
pub use svg_path::SvgPath;

use std::fmt::Write;

use crate::geo::{Coordinate, Degrees, Distance, Share};

pub use axis::Axis;
pub use pixels::{Halving, Pixels};
pub use plane_point::PlanePoint;
pub use scale::Scale;
use tenths::Tenths;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapFrame {
    south_west: Coordinate,
    east: Distance,
    north: Distance,
    width: Pixels,
}

impl MapFrame {
    const MARGIN: Share = Share::new(0.08);
    const MINIMUM_SPAN: Distance = Distance::new(300.0);

    #[must_use]
    pub fn around(points: &[Coordinate], width: Pixels) -> Option<MapFrame> {
        let first = points.first()?;
        let south = points
            .iter()
            .map(|point| point.lat().angle().degrees())
            .fold(first.lat().angle().degrees(), f64::min);
        let north = points
            .iter()
            .map(|point| point.lat().angle().degrees())
            .fold(first.lat().angle().degrees(), f64::max);
        let west = points
            .iter()
            .map(|point| point.lon().angle().degrees())
            .fold(first.lon().angle().degrees(), f64::min);
        let east = points
            .iter()
            .map(|point| point.lon().angle().degrees())
            .fold(first.lon().angle().degrees(), f64::max);
        let corner = Coordinate::new(Degrees::new(south), Degrees::new(west));
        let span_east = Coordinate::new(Degrees::new(south), Degrees::new(east))
            .lon()
            .east_of(corner)
            .max(Self::MINIMUM_SPAN);
        let span_north = Coordinate::new(Degrees::new(north), Degrees::new(west))
            .lat()
            .north_of(corner)
            .max(Self::MINIMUM_SPAN);
        let pad = span_east.max(span_north) * Self::MARGIN;
        Some(MapFrame {
            south_west: corner.offset_by(-pad, -pad),
            east: span_east + (pad + pad),
            north: span_north + (pad + pad),
            width,
        })
    }

    #[must_use]
    pub fn zoomed_out(self, scale: Scale) -> MapFrame {
        let east = self.east * scale.factor();
        let north = self.north * scale.factor();
        MapFrame {
            south_west: self.south_west.offset_by(
                -(east - self.east) * Share::HALF,
                -(north - self.north) * Share::HALF,
            ),
            east,
            north,
            ..self
        }
    }

    #[must_use]
    pub fn shifted(self, right: Pixels, down: Pixels) -> MapFrame {
        MapFrame {
            south_west: self
                .south_west
                .offset_by(-self.distance_of(right), self.distance_of(down)),
            ..self
        }
    }

    #[must_use]
    pub fn extent(self, axis: Axis) -> Pixels {
        match axis {
            Axis::Across => self.width,
            Axis::Down => self.width * (self.north / self.east),
        }
    }

    #[must_use]
    pub fn span(self) -> Distance {
        self.east
    }

    #[must_use]
    pub fn project(self, point: Coordinate) -> PlanePoint {
        let height = self.extent(Axis::Down);
        PlanePoint::new(
            self.width * (point.lon().east_of(self.south_west) / self.east),
            height - height * (point.lat().north_of(self.south_west) / self.north),
        )
    }

    #[must_use]
    pub fn length_on_map(self, distance: Distance) -> Pixels {
        self.width * (distance / self.east)
    }

    fn distance_of(self, pixels: Pixels) -> Distance {
        self.east * (pixels / self.width)
    }

    #[must_use]
    pub fn svg_path(self, points: &[Coordinate]) -> SvgPath {
        SvgPath::new(
            points
                .iter()
                .enumerate()
                .fold(String::new(), |mut path, (index, point)| {
                    let projected = self.project(*point);
                    let command = if index == 0 { 'M' } else { 'L' };
                    let _ = write!(
                        path,
                        "{command}{} {} ",
                        Tenths::rounded_from(projected[Axis::Across].pixels()),
                        Tenths::rounded_from(projected[Axis::Down].pixels())
                    );
                    path
                }),
        )
    }
}

impl<Holder: AsRef<MapFrame>> From<&Holder> for MapFrame {
    fn from(holder: &Holder) -> MapFrame {
        *holder.as_ref()
    }
}

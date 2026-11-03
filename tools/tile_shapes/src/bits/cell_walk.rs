use guide::end::End;
use guide::leg::Leg;
use map_class::Zoom;
use tiles::GridAxis;
use tiles::TileId;

use super::WorldPoint;

pub struct CellWalk;

impl CellWalk {
    const SEGMENT_POINTS: u32 = 8;
    const REACH: [i32; 3] = [-1, 0, 1];

    #[must_use]
    pub fn cells_along(leg: &Leg, zoom: Zoom) -> Vec<TileId> {
        Self::dense_points(leg)
            .into_iter()
            .map(|point| point.cell_at(zoom))
            .flat_map(Self::cells_around)
            .collect()
    }

    fn cells_around(cell: TileId) -> Vec<TileId> {
        Self::REACH
            .into_iter()
            .flat_map(|right| {
                Self::REACH.into_iter().map(move |down| {
                    TileId::new(
                        cell.zoom(),
                        cell.index_along(GridAxis::Across)
                            .saturating_add_signed(right),
                        cell.index_along(GridAxis::Down).saturating_add_signed(down),
                    )
                })
            })
            .collect()
    }

    fn dense_points(leg: &Leg) -> Vec<WorldPoint> {
        let drawn = leg.line().points();
        let points: Vec<WorldPoint> = if drawn.len() > 1 {
            drawn
                .iter()
                .copied()
                .map(WorldPoint::of_coordinate)
                .collect()
        } else {
            [End::Start, End::Finish]
                .into_iter()
                .map(|end| WorldPoint::of_coordinate(leg.point(end)))
                .collect()
        };
        points
            .iter()
            .zip(points.iter().skip(1))
            .flat_map(|(start, finish)| {
                (0..Self::SEGMENT_POINTS).map(|index| {
                    start.toward(*finish, f64::from(index) / f64::from(Self::SEGMENT_POINTS))
                })
            })
            .chain(points.last().copied())
            .collect()
    }
}

use crate::WorldPoint;
use tiles::GridAxis;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GridPoint {
    across: i64,
    down: i64,
}

impl GridPoint {
    #[must_use]
    pub fn path_from(points: &[WorldPoint]) -> Vec<GridPoint> {
        let mut grid: Vec<GridPoint> = points.iter().copied().map(GridPoint::from).collect();
        grid.dedup();
        grid
    }

    #[must_use]
    pub fn ring_from(points: &[WorldPoint]) -> Option<Vec<GridPoint>> {
        let path = Self::path_from(points);
        let open = path
            .split_last()
            .filter(|(last, _)| path.first() == Some(last))
            .map_or(path.as_slice(), |(_, rest)| rest);
        (open.len() > 2 && Self::doubled_area(open) != 0).then(|| open.to_vec())
    }

    #[must_use]
    pub fn doubled_area(ring: &[GridPoint]) -> i64 {
        ring.iter()
            .zip(ring.iter().cycle().skip(1))
            .map(|(start, finish)| start.across * finish.down - finish.across * start.down)
            .sum()
    }

    #[must_use]
    pub fn step_from(self, last: GridPoint) -> [u64; 2] {
        [
            Self::zigzag(self.across - last.across),
            Self::zigzag(self.down - last.down),
        ]
    }

    const fn zigzag(step: i64) -> u64 {
        ((step << 1) ^ (step >> 63)).cast_unsigned()
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "tile units stay within a few thousand of the tile"
    )]
    const fn whole(value: f64) -> i64 {
        value.round_ties_even() as i64
    }
}

impl From<WorldPoint> for GridPoint {
    fn from(point: WorldPoint) -> GridPoint {
        GridPoint {
            across: Self::whole(point.along(GridAxis::Across)),
            down: Self::whole(point.along(GridAxis::Down)),
        }
    }
}

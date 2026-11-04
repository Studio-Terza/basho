use crate::WorldPoint;
use tiles::GridAxis;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl ClipSide {
    pub const ALL: [ClipSide; 4] = [
        ClipSide::Left,
        ClipSide::Right,
        ClipSide::Top,
        ClipSide::Bottom,
    ];
    const LEAST: f64 = -64.0;
    const MOST: f64 = WorldPoint::EXTENT + 64.0;

    #[must_use]
    pub fn clipped(self, ring: &[WorldPoint]) -> Vec<WorldPoint> {
        ring.iter()
            .zip(ring.iter().cycle().skip(1))
            .flat_map(|(start, finish)| self.kept_pair(*start, *finish))
            .collect()
    }

    fn kept_pair(self, start: WorldPoint, finish: WorldPoint) -> Vec<WorldPoint> {
        let before = self.room(start);
        let after = self.room(finish);
        let crossing = || start.toward(finish, before / (before - after));
        match [before >= 0.0, after >= 0.0] {
            [true, true] => vec![start],
            [true, false] => vec![start, crossing()],
            [false, true] => vec![crossing()],
            [false, false] => Vec::new(),
        }
    }

    fn room(self, point: WorldPoint) -> f64 {
        match self {
            ClipSide::Left => point.along(GridAxis::Across) - Self::LEAST,
            ClipSide::Right => Self::MOST - point.along(GridAxis::Across),
            ClipSide::Top => point.along(GridAxis::Down) - Self::LEAST,
            ClipSide::Bottom => Self::MOST - point.along(GridAxis::Down),
        }
    }
}

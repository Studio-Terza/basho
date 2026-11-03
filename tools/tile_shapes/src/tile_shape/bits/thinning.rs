use crate::WorldPoint;
use map_class::Zoom;
use tiles::GridAxis;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Thinning {
    tolerance: f64,
}

impl Thinning {
    pub const FINE: Thinning = Thinning { tolerance: 1.0 };
    pub const COARSE: Thinning = Thinning { tolerance: 4.0 };

    #[must_use]
    pub fn drawn_at(path: &[WorldPoint], zoom: Zoom) -> Vec<WorldPoint> {
        let scaled: Vec<WorldPoint> = path.iter().map(|point| point.in_tile_units(zoom)).collect();
        if zoom == Zoom::MOST {
            Self::FINE.thinned(&scaled)
        } else {
            Self::COARSE.thinned(&scaled)
        }
    }

    #[must_use]
    pub fn thinned(self, path: &[WorldPoint]) -> Vec<WorldPoint> {
        match path {
            [first, middle @ .., last] if !middle.is_empty() => {
                let farthest = middle
                    .iter()
                    .map(|point| Self::offset_of(*point, *first, *last))
                    .enumerate()
                    .max_by(|left, right| left.1.total_cmp(&right.1))
                    .filter(|found| found.1 > self.tolerance * self.tolerance);
                farthest.map_or_else(
                    || vec![*first, *last],
                    |found| self.split_at_index(path, found.0 + 1),
                )
            }
            _ => path.to_vec(),
        }
    }

    fn split_at_index(self, path: &[WorldPoint], index: usize) -> Vec<WorldPoint> {
        let before = self.thinned(&path[..=index]);
        let after = self.thinned(&path[index..]);
        before.iter().chain(after.iter().skip(1)).copied().collect()
    }

    fn offset_of(point: WorldPoint, start: WorldPoint, finish: WorldPoint) -> f64 {
        let along = finish - start;
        let from = point - start;
        let across = along.along(GridAxis::Across);
        let down = along.along(GridAxis::Down);
        let length = across.powi(2) + down.powi(2);
        let share = if length == 0.0 {
            0.0
        } else {
            ((from.along(GridAxis::Across) * across + from.along(GridAxis::Down) * down) / length)
                .clamp(0.0, 1.0)
        };
        (from.along(GridAxis::Across) - share * across).powi(2)
            + (from.along(GridAxis::Down) - share * down).powi(2)
    }
}

use std::cmp::Ordering;

use crate::geo::{Coordinate, Distance};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trail {
    points: Vec<Coordinate>,
}

impl Trail {
    pub const SPACING: Distance = Distance::new(10.0);
    pub const CAPACITY: usize = 1000;

    #[must_use]
    pub fn recorded(self, fix: Coordinate) -> Trail {
        match self.points.last().map(|last| Gap::between(*last, fix)) {
            Some(Gap::Close) => self,
            Some(Gap::Clear) | None => Trail {
                points: self
                    .points
                    .iter()
                    .copied()
                    .chain(std::iter::once(fix))
                    .skip((self.points.len() + 1).saturating_sub(Self::CAPACITY))
                    .collect(),
            },
        }
    }
}

impl Trail {
    #[must_use]
    pub fn points(&self) -> &[Coordinate] {
        &self.points
    }
}

impl FromIterator<Coordinate> for Trail {
    fn from_iter<I: IntoIterator<Item = Coordinate>>(fixes: I) -> Trail {
        fixes.into_iter().fold(Trail::default(), Trail::recorded)
    }
}

enum Gap {
    Close,
    Clear,
}

impl Gap {
    fn between(last: Coordinate, fix: Coordinate) -> Gap {
        match last.distance_to(fix).cmp(&Trail::SPACING) {
            Ordering::Less => Gap::Close,
            Ordering::Equal | Ordering::Greater => Gap::Clear,
        }
    }
}

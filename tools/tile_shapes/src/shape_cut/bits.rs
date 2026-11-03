use crate::GridPoint;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathKind {
    Spot = 1,
    Line = 2,
    Area = 3,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShapeCut {
    kind: PathKind,
    paths: Vec<Vec<GridPoint>>,
}

impl ShapeCut {
    #[must_use]
    pub fn new(kind: PathKind, paths: Vec<Vec<GridPoint>>) -> Option<ShapeCut> {
        (!paths.is_empty()).then_some(ShapeCut { kind, paths })
    }

    #[must_use]
    pub fn kind(&self) -> PathKind {
        self.kind
    }

    #[must_use]
    pub fn paths(&self) -> &[Vec<GridPoint>] {
        &self.paths
    }
}

use crate::geo::{Coordinate, RouteLine};

#[derive(derive_more::AsRef, Clone, Debug, PartialEq)]
pub struct NoMappedPath {
    #[as_ref]
    line: RouteLine,
}

impl NoMappedPath {
    pub(super) fn spanning(points: Vec<Coordinate>) -> NoMappedPath {
        NoMappedPath {
            line: RouteLine::new(points),
        }
    }
}

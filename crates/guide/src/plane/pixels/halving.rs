use crate::geo::Share;
use crate::plane::Pixels;

pub trait Halving {
    #[must_use]
    fn half(self) -> Self;
    #[must_use]
    fn twice(self) -> Self;
}

impl Halving for Pixels {
    fn half(self) -> Pixels {
        self * Share::HALF.fraction()
    }

    fn twice(self) -> Pixels {
        self + self
    }
}

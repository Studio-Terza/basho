use newtype::Newtype;
use std::ops::Div;

#[derive(Newtype, newtype::Arbitrary, Clone, Copy, Debug, PartialEq, PartialOrd)]
#[newtype(strategy = 0.0..=1.0)]
pub struct Share(f64);

impl Share {
    pub const HALF: Share = Share(0.5);

    #[must_use]
    pub fn ratio<Amount: Div<Output = f64> + PartialOrd + Default>(
        part: Amount,
        whole: Amount,
    ) -> Share {
        Share(if whole > Amount::default() {
            (part / whole).clamp(0.0, 1.0)
        } else {
            0.0
        })
    }

    #[must_use]
    pub const fn fraction(self) -> f64 {
        self.0
    }
}

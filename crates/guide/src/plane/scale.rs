use newtype::Newtype;
#[derive(Newtype, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Scale(f64);

impl Scale {
    #[must_use]
    #[inline]
    pub const fn factor(self) -> f64 {
        self.0
    }
}

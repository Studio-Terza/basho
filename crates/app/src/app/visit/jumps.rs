use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Jumps(u32);

impl Jumps {
    #[must_use]
    pub fn next(self) -> Jumps {
        Jumps(self.0.wrapping_add(1))
    }
}

impl fmt::Display for Jumps {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "jump-{}", self.0)
    }
}

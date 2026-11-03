use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhaseLength(u16);

impl PhaseLength {
    pub const SIMULATED: PhaseLength = PhaseLength(20);
}

impl fmt::Display for PhaseLength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

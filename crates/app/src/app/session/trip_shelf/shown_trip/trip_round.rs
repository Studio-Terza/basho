#[derive(newtype::Display, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TripRound(u32);

impl TripRound {
    #[must_use]
    pub fn advanced(self) -> TripRound {
        TripRound(self.0.wrapping_add(1))
    }
}

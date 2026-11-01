#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Days(f64);

impl Days {
    #[must_use]
    pub const fn new(days: f64) -> Days {
        Days(days)
    }

    #[must_use]
    pub const fn days(self) -> f64 {
        self.0
    }
}

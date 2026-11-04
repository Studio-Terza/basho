use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tenths(i64);

impl Tenths {
    const PER_UNIT: i64 = 10;
    const PER_UNIT_SCALE: f64 = 10.0;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "a map coordinate in tenths of a pixel is far inside i64"
    )]
    #[must_use]
    pub const fn rounded_from(value: f64) -> Tenths {
        Tenths((value * Self::PER_UNIT_SCALE).round_ties_even() as i64)
    }
}

impl fmt::Display for Tenths {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = Some(self.0)
            .filter(|tenths| *tenths < 0)
            .map_or("", |_| "-");
        let whole = self.0.unsigned_abs();
        let per_unit = Self::PER_UNIT.unsigned_abs();
        write!(formatter, "{sign}{}.{}", whole / per_unit, whole % per_unit)
    }
}

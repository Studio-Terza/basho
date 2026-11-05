use std::time::Duration;
use std::time::Instant;

use guide::plane::Pixels;
use menu::Motion;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Flick(f64);

impl Flick {
    pub const COMMIT: Flick = Flick(0.2);
    const NEWEST: f64 = 0.7;
    const STILL: Pixels = Pixels::new(0.5);
    const MILLIS_PER_SECOND: f64 = 1000.0;

    pub fn followed(self, moved: Pixels, from: Instant, to: Instant) -> Flick {
        let elapsed = to.saturating_duration_since(from);
        let millis = elapsed.as_secs_f64() * Self::MILLIS_PER_SECOND;
        if elapsed > Duration::from(Motion::Rest) {
            Flick(moved.pixels() / millis)
        } else if moved.abs() < Self::STILL {
            self
        } else if millis > 0.0 {
            Flick(Self::NEWEST * (moved.pixels() / millis) + (1.0 - Self::NEWEST) * self.0)
        } else {
            self
        }
    }

    pub fn reversed(self) -> Flick {
        Flick(-self.0)
    }
}

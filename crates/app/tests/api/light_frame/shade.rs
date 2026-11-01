use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shade([u8; 3]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Brightness(f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Contrast(f64);

impl Brightness {
    pub const fn new(level: f64) -> Brightness {
        Brightness(level)
    }
}

impl Contrast {
    pub const fn new(ratio: f64) -> Contrast {
        Contrast(ratio)
    }
}

impl Shade {
    const LUMA: [f64; 3] = [0.2126, 0.7152, 0.0722];
    const LINEAR_KNEE: f64 = 0.040_45;
    const LINEAR_SLOPE: f64 = 12.92;
    const CURVE_OFFSET: f64 = 0.055;
    const CURVE_SCALE: f64 = 1.055;
    const CURVE_POWER: f64 = 2.4;
    const FLARE: f64 = 0.05;
    const CHANNEL_MAX: f64 = 255.0;

    pub fn brightness(self) -> Brightness {
        Brightness(
            self.0
                .iter()
                .zip(Self::LUMA)
                .map(|(channel, weight)| {
                    weight * Self::linear(f64::from(*channel) / Self::CHANNEL_MAX)
                })
                .sum(),
        )
    }

    pub fn contrast_with(self, other: Shade) -> Contrast {
        let (mine, theirs) = (self.brightness().0, other.brightness().0);
        let (light, dark) = (mine.max(theirs), mine.min(theirs));
        Contrast((light + Self::FLARE) / (dark + Self::FLARE))
    }

    pub fn is_grey(self) -> bool {
        self.0.iter().all(|channel| *channel == self.0[0])
    }

    fn linear(channel: f64) -> f64 {
        if channel <= Self::LINEAR_KNEE {
            channel / Self::LINEAR_SLOPE
        } else {
            ((channel + Self::CURVE_OFFSET) / Self::CURVE_SCALE).powf(Self::CURVE_POWER)
        }
    }
}

impl FromStr for Shade {
    type Err = std::num::ParseIntError;

    fn from_str(text: &str) -> Result<Shade, Self::Err> {
        let digits = text.trim().trim_start_matches('#');
        let channel = |at: usize| u8::from_str_radix(&digits[at..at + 2], 16);
        Ok(Shade([channel(0)?, channel(2)?, channel(4)?]))
    }
}

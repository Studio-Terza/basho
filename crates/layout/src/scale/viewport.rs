use guide::geo::Share;
use guide::map_frame::{Pixels, Scale};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    width: Pixels,
    reading: Scale,
}

impl Viewport {
    const PHONE: Viewport = Viewport {
        width: Pixels::new(430.0),
        reading: Scale::new(1.0),
    };
    const SYSTEM_BODY: Pixels = Pixels::new(17.0);
    const LARGEST_READING: Scale = Scale::new(1.6);

    #[must_use]
    pub fn at_width(self, width: Pixels) -> Viewport {
        Viewport { width, ..self }
    }

    #[must_use]
    pub fn reading_of(body: Pixels) -> Scale {
        Scale::new(
            (body / Self::SYSTEM_BODY)
                .clamp(Self::PHONE.reading.factor(), Self::LARGEST_READING.factor()),
        )
    }

    #[must_use]
    pub fn read_at(self, reading: Scale) -> Viewport {
        Viewport { reading, ..self }
    }

    #[must_use]
    pub fn reading(self) -> Scale {
        self.reading
    }

    #[must_use]
    pub fn scaled(self, length: Pixels) -> Pixels {
        length * (self.width / Self::PHONE.width)
    }

    #[must_use]
    pub fn across(self, share: Share) -> Pixels {
        self.width * share.fraction()
    }
}

impl Default for Viewport {
    fn default() -> Viewport {
        Viewport::PHONE
    }
}

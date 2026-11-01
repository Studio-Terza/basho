use guide::geo::Share;
use guide::map_frame::Pixels;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tracking {
    Normal,
    Wide,
    Widest,
}

impl Tracking {
    const NORMAL: Share = Share::new(0.0);
    const WIDE: Share = Share::new(0.1);
    const WIDEST: Share = Share::new(0.19);

    #[must_use]
    pub fn spread(self, size: Pixels) -> Pixels {
        size * match self {
            Tracking::Normal => Self::NORMAL,
            Tracking::Wide => Self::WIDE,
            Tracking::Widest => Self::WIDEST,
        }
        .fraction()
    }
}

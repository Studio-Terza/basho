use guide::geo::{Degrees, Share};
use guide::map_frame::{Pixels, Scale};

pub struct Fit;

impl Fit {
    pub const ZOOM_STEP: Scale = Scale::new(1.06);
    pub const ZOOM_SEARCH_LENGTH: usize = 24;
    pub const POINTER_CAPACITY: usize = 3;
    pub const POINTER_SPACING: Scale = Scale::new(5.0);
    pub const CALLOUT_SQUASH: Share = Share::new(0.6);
    const CALLOUT_STEP: Degrees = Degrees::new(30.0);
    const FULL_TURN: Degrees = Degrees::new(360.0);
    const CALLOUT_REACHES: [Scale; 4] = [
        Scale::new(0.0),
        Scale::new(1.0),
        Scale::new(2.0),
        Scale::new(3.0),
    ];
    pub const MERGE_REACHES: [Scale; 6] = [
        Scale::new(1.0),
        Scale::new(2.0),
        Scale::new(3.0),
        Scale::new(4.0),
        Scale::new(6.0),
        Scale::new(8.0),
    ];
    pub const FRAME_SHIFTS: [Scale; 9] = [
        Scale::new(0.0),
        Scale::new(1.0),
        Scale::new(-1.0),
        Scale::new(2.0),
        Scale::new(-2.0),
        Scale::new(3.0),
        Scale::new(-3.0),
        Scale::new(4.0),
        Scale::new(-4.0),
    ];
    pub const POINTER_NUDGES: [Scale; 5] = [
        Scale::new(0.0),
        Scale::new(1.0),
        Scale::new(-1.0),
        Scale::new(2.0),
        Scale::new(-2.0),
    ];

    #[must_use]
    pub fn callout_angles() -> Vec<Degrees> {
        std::iter::successors(Some(Degrees::new(0.0)), |angle| {
            Some(*angle + Self::CALLOUT_STEP)
        })
        .take_while(|angle| angle.degrees() < Self::FULL_TURN.degrees())
        .collect()
    }

    #[must_use]
    pub fn callout_reaches(nearest: Pixels, step: Pixels) -> Vec<Pixels> {
        Self::CALLOUT_REACHES
            .iter()
            .map(|times| nearest + step * times.factor())
            .collect()
    }

    #[must_use]
    pub fn scaled(times: &[Scale], step: Pixels) -> Vec<Pixels> {
        times.iter().map(|times| step * times.factor()).collect()
    }
}

use guide::guidance::Phase;
use guide::leg::Mode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepPart {
    Walk,
    Board,
    Alight,
}

impl StepPart {
    const WALK: [StepPart; 1] = [StepPart::Walk];
    const RIDE: [StepPart; 2] = [StepPart::Board, StepPart::Alight];

    #[must_use]
    pub fn for_mode(mode: Mode) -> &'static [StepPart] {
        match mode {
            Mode::Walk => &Self::WALK,
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => &Self::RIDE,
        }
    }

    #[must_use]
    pub fn during(mode: Mode, phase: Phase) -> StepPart {
        if mode == Mode::Walk {
            StepPart::Walk
        } else if phase == Phase::Moving || phase == Phase::Done {
            StepPart::Alight
        } else {
            StepPart::Board
        }
    }
}

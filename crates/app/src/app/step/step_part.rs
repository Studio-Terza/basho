use guide::guidance::Phase;
use guide::guidance::tracking::Stage;
use guide::leg::Mode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepPart {
    Walk,
    Board,
    Alight,
}

impl StepPart {
    const WALK: [StepPart; 1] = [StepPart::Walk];
    const ALIGHTING: [Phase; 2] = [Phase::Moving, Phase::Done];
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
        match mode {
            Mode::Walk => StepPart::Walk,
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Self::riding(phase),
        }
    }

    fn riding(phase: Phase) -> StepPart {
        Some(phase)
            .filter(|phase| Self::ALIGHTING.contains(phase))
            .map_or(StepPart::Board, |_| StepPart::Alight)
    }

    #[must_use]
    pub fn stage(self) -> Stage {
        match self {
            StepPart::Board => Stage::Boarding,
            StepPart::Walk | StepPart::Alight => Stage::Under,
        }
    }
}

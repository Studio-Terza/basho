use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cadence {
    Clock,
    PairHold,
    DecisionMemory,
    #[cfg(target_os = "ios")]
    Location,
    #[cfg(target_os = "ios")]
    FixAge,
    #[cfg(target_os = "ios")]
    Heading,
}

impl Cadence {
    const CLOCK: Duration = Duration::from_secs(2);
    const PAIR_HOLD: Duration = Duration::from_millis(800);
    const DECISION_MEMORY: Duration = Duration::from_mins(5);
    #[cfg(target_os = "ios")]
    const LOCATION: Duration = Duration::from_secs(3);
    #[cfg(target_os = "ios")]
    const FIX_AGE: Duration = Duration::from_secs(30);
    #[cfg(target_os = "ios")]
    const HEADING: Duration = Duration::from_secs(3);

    pub fn every(self) -> Duration {
        match self {
            Cadence::Clock => Self::CLOCK,
            Cadence::PairHold => Self::PAIR_HOLD,
            Cadence::DecisionMemory => Self::DECISION_MEMORY,
            #[cfg(target_os = "ios")]
            Cadence::Location => Self::LOCATION,
            #[cfg(target_os = "ios")]
            Cadence::FixAge => Self::FIX_AGE,
            #[cfg(target_os = "ios")]
            Cadence::Heading => Self::HEADING,
        }
    }
}

use guide::DeadlineKind;
use guide::guidance::timeline::Buffer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wanted {
    Ride,
    RideWithBackup,
    Cutoff,
}

impl Wanted {
    pub fn kept(self, buffer: Buffer<'_>) -> Option<Buffer<'_>> {
        match self {
            Wanted::Ride => Self::named_ride(buffer),
            Wanted::RideWithBackup => Self::named_ride(buffer)
                .filter(|buffer| buffer.backup().is_some() && buffer.reaches_before_event()),
            Wanted::Cutoff => Some(buffer).filter(|buffer| !buffer.deadline().ends_stretch()),
        }
    }

    pub fn named_ride(buffer: Buffer<'_>) -> Option<Buffer<'_>> {
        Some(buffer)
            .filter(|buffer| buffer.deadline().kind() == DeadlineKind::Ride)
            .filter(|buffer| {
                buffer
                    .deadline()
                    .place()
                    .segments()
                    .iter()
                    .any(|segment| segment.reading().is_some())
            })
    }
}

use guide::DeadlineKind;
use guide::StretchEnd;
use guide::guidance::timeline::{Buffer, Reaching};

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
                .filter(|buffer| buffer.backup().is_some())
                .filter(|buffer| buffer.reaches_before_event() == Reaching::InTime),
            Wanted::Cutoff => Self::open_ended(buffer),
        }
    }

    fn open_ended(buffer: Buffer<'_>) -> Option<Buffer<'_>> {
        match buffer.deadline().kind().ending() {
            StretchEnd::Continues => Some(buffer),
            StretchEnd::Ends => None,
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

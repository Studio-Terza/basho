use std::fmt;

use guide::day_number::DayNumber;
use guide::guidance::tracking::{Placing, Tracking};
use guide::guidance::{Guidance, LegProgress, PositionSource};
use guide::timeline::Stance;

#[derive(strum::Display, Clone, Copy, Debug, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
enum RouteSide {
    OnRoute,
    OffRoute,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrackerDecision {
    day: DayNumber,
    placing: Placing,
    source: PositionSource,
    stance: Stance,
    side: RouteSide,
    tracking: Tracking,
}

impl TrackerDecision {
    #[must_use]
    pub fn seen_in(guidance: &Guidance<'_>, progress: LegProgress) -> TrackerDecision {
        let day = guidance.day().number();
        TrackerDecision {
            day,
            placing: guidance.placing(),
            source: guidance.source(),
            stance: guidance.phase().stance(),
            side: guidance
                .off_route()
                .map_or(RouteSide::OnRoute, |_| RouteSide::OffRoute),
            tracking: progress
                .for_day(day)
                .map_or(Tracking::Automatic, LegProgress::tracking),
        }
    }
}

impl fmt::Display for TrackerDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = match self.source {
            PositionSource::Gps => "gps",
            PositionSource::Estimate => "estimate",
            PositionSource::GpsLost { .. } => "gps-lost",
        };
        let control = self.tracking.floor().map_or("automatic", |_| "manual");
        write!(
            formatter,
            "day {} leg {} {:?} {source} {} {control}{}",
            self.day, self.placing, self.stance, self.side, self.tracking
        )
    }
}

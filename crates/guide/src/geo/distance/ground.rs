use crate::geo::Distance;
use crate::map_frame::Scale;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ground {
    #[cfg(target_os = "ios")]
    FixSpread,
    #[cfg(target_os = "ios")]
    FixStep,
    Beside,
    RoundKilometers,
    RangesFrom,
    LocalDetail,
    StarReach,
    SeaReach,
    PeakReach,
}

impl Ground {
    const DECADES: [Distance; 4] = [
        Distance::new(100.0),
        Distance::new(1_000.0),
        Distance::new(10_000.0),
        Distance::new(100_000.0),
    ];
    const MANTISSAS: [Scale; 3] = [Scale::new(1.0), Scale::new(2.0), Scale::new(5.0)];
    #[cfg(target_os = "ios")]
    const FIX_SPREAD: Distance = Distance::new(100.0);
    #[cfg(target_os = "ios")]
    const FIX_STEP: Distance = Distance::new(10.0);
    const BESIDE: Distance = Distance::new(10.0);
    const ROUND_KILOMETERS: Distance = Distance::new(10_000.0);
    const RANGES_FROM: Distance = Distance::new(25_000.0);
    const LOCAL_DETAIL: Distance = Distance::new(30_000.0);
    const SEA_REACH: Distance = Distance::new(60_000.0);
    const PEAK_REACH: Distance = Distance::new(70_000.0);

    #[must_use]
    pub fn scale_steps() -> Vec<Distance> {
        Self::DECADES
            .iter()
            .flat_map(|decade| {
                Self::MANTISSAS
                    .iter()
                    .map(move |mantissa| *decade * mantissa.factor())
            })
            .filter(|step| Self::DECADES.last().is_some_and(|widest| step <= widest))
            .collect()
    }
}

impl From<Ground> for Distance {
    fn from(ground: Ground) -> Distance {
        match ground {
            #[cfg(target_os = "ios")]
            Ground::FixSpread => Ground::FIX_SPREAD,
            #[cfg(target_os = "ios")]
            Ground::FixStep => Ground::FIX_STEP,
            Ground::Beside => Ground::BESIDE,
            Ground::RoundKilometers => Ground::ROUND_KILOMETERS,
            Ground::RangesFrom => Ground::RANGES_FROM,
            Ground::LocalDetail | Ground::StarReach => Ground::LOCAL_DETAIL,
            Ground::SeaReach => Ground::SEA_REACH,
            Ground::PeakReach => Ground::PEAK_REACH,
        }
    }
}

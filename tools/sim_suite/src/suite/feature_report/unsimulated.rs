use crate::suite::{Feature, Standing};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Unsimulated {
    feature: Feature,
    what: &'static str,
    why: &'static str,
    standing: Standing,
}

impl Unsimulated {
    const ALL: [Unsimulated; 5] = [
        Unsimulated::new(
            Feature::TripPicker,
            "a file from iCloud Drive or another app",
            "no account is signed in; the picker is proved on On My iPhone",
        ),
        Unsimulated::new(
            Feature::BugReport,
            "real GPS accuracy in a report",
            "simulated fixes always read ± 10 m",
        ),
        Unsimulated::new(
            Feature::Maps,
            "drawing on the phone's GPU",
            "the simulator draws WebGL with the Mac's GPU",
        ),
        Unsimulated::new(
            Feature::SetNow,
            "the compass",
            "the simulator has no heading hardware",
        ),
        Unsimulated::new(
            Feature::MapPerf,
            "battery drain",
            "the simulator has no battery; the figures are the Mac's",
        ),
    ];

    const fn new(feature: Feature, what: &'static str, why: &'static str) -> Unsimulated {
        Unsimulated {
            feature,
            what,
            why,
            standing: Standing::CannotSimulate,
        }
    }

    #[must_use]
    pub fn listed_for(feature: Feature) -> Vec<Unsimulated> {
        Self::ALL
            .into_iter()
            .filter(|gap| gap.feature == feature)
            .collect()
    }
}

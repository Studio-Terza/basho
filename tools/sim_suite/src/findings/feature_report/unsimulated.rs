use crate::findings::wording::Wording;
use crate::suite::{Feature, Standing};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Unsimulated {
    feature: Feature,
    what: Wording,
    why: Wording,
    standing: Standing,
}

impl Unsimulated {
    const ALL: [Unsimulated; 5] = [
        Unsimulated::new(
            Feature::TripPicker,
            Wording::new("a file from iCloud Drive or another app"),
            Wording::new("no account is signed in; the picker is proved on On My iPhone"),
        ),
        Unsimulated::new(
            Feature::BugReport,
            Wording::new("real GPS accuracy in a report"),
            Wording::new("simulated fixes always read ± 10 m"),
        ),
        Unsimulated::new(
            Feature::Maps,
            Wording::new("drawing on the phone's GPU"),
            Wording::new("the simulator draws WebGL with the Mac's GPU"),
        ),
        Unsimulated::new(
            Feature::SetNow,
            Wording::new("the compass"),
            Wording::new("the simulator has no heading hardware"),
        ),
        Unsimulated::new(
            Feature::MapPerf,
            Wording::new("battery drain"),
            Wording::new("the simulator has no battery; the figures are the Mac's"),
        ),
    ];

    const fn new(feature: Feature, what: Wording, why: Wording) -> Unsimulated {
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

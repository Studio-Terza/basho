use crate::findings::map_phase::TopSample;
use crate::suite::SuiteWord;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct PhaseFigures {
    phase: SuiteWord,
    #[serde(flatten)]
    sample: TopSample,
}

impl From<(SuiteWord, TopSample)> for PhaseFigures {
    fn from((phase, sample): (SuiteWord, TopSample)) -> PhaseFigures {
        PhaseFigures { phase, sample }
    }
}

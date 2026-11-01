use crate::evidence::{EvidenceCounts, EvidenceTally};
use crate::itinerary::Finding;
use crate::trip::{Trip, TripError};

#[derive(newtype::Newtype, Clone, Copy, Debug)]
pub struct TripSource<'text>(&'text str);

impl<'text> TripSource<'text> {
    const TABLE_OPEN: &'static str = "\n[";
    const STRETCH_OPENS: [&'static str; 4] = [
        "\n[[days]]\n",
        "\n[[routes]]\n",
        "\n[kana]\n",
        "\n[[catches]]\n",
    ];

    fn opens_stretch(self, index: usize) -> bool {
        self.0.get(index..).is_some_and(|rest| {
            Self::STRETCH_OPENS
                .iter()
                .any(|open| rest.starts_with(open))
        })
    }

    #[must_use]
    pub fn stretches(self) -> Vec<TripSource<'text>> {
        let mut cuts: Vec<usize> = self
            .0
            .match_indices(Self::TABLE_OPEN)
            .filter(|(at, _)| self.opens_stretch(*at))
            .map(|(at, _)| at + 1)
            .collect();
        cuts.extend([0, self.0.len()]);
        cuts.sort_unstable();
        cuts.iter()
            .zip(cuts.iter().skip(1))
            .filter_map(|(from, to)| self.0.get(*from..*to))
            .filter(|stretch| !stretch.is_empty())
            .map(TripSource)
            .collect()
    }

    pub fn trip(self) -> Result<Trip, TripError> {
        self.0.parse()
    }

    pub fn evidence_counts(self) -> Result<EvidenceCounts, TripError> {
        self.0
            .parse::<EvidenceTally>()
            .map(EvidenceCounts::from)
            .map_err(TripError::Toml)
    }

    #[must_use]
    pub fn findings(self) -> Vec<Finding> {
        match self.trip() {
            Ok(trip) => trip.unresolved_names(),
            Err(TripError::Invalid(findings)) => findings,
            Err(error) => vec![Finding::whole(error)],
        }
    }
}

impl AsRef<str> for TripSource<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

use std::fmt;

use guide::itinerary::Finding;

use crate::task::failure::Failure;
use crate::task::problem_count::ProblemCount;

pub struct CheckedTrip(Vec<Finding>);

impl CheckedTrip {
    pub fn verdict(&self) -> Result<(), Failure> {
        self.0.first().map_or(Ok(()), |_| {
            Err(Failure::Trip(ProblemCount::from(self.0.as_slice())))
        })
    }
}

impl From<Vec<Finding>> for CheckedTrip {
    fn from(findings: Vec<Finding>) -> CheckedTrip {
        CheckedTrip(findings)
    }
}

impl fmt::Display for CheckedTrip {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .try_for_each(|finding| writeln!(formatter, "{finding}"))?;
        write!(formatter, "trip-check: {} problems", self.0.len())
    }
}

use crate::itinerary::Finding;
use crate::trip::TripError;

pub struct Findings(Vec<Finding>);

impl FromIterator<Finding> for Findings {
    fn from_iter<Found: IntoIterator<Item = Finding>>(found: Found) -> Findings {
        Findings(found.into_iter().collect())
    }
}

impl Findings {
    pub fn into_result<Clean>(self, clean: Clean) -> Result<Clean, TripError> {
        match self.0.first() {
            Some(_) => Err(TripError::Invalid(self.0)),
            None => Ok(clean),
        }
    }
}

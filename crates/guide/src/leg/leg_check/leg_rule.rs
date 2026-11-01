use crate::itinerary::Problem;
use crate::leg::Leg;

pub trait LegRule {
    fn problems(&self, leg: &Leg) -> Vec<Problem>;
}

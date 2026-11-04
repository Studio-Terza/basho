use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0} is not a finite number")]
pub struct NotFinite(f64);

impl NotFinite {
    pub fn ensure_finite(number: f64) -> Result<f64, NotFinite> {
        number
            .is_finite()
            .then_some(number)
            .ok_or(NotFinite(number))
    }
}

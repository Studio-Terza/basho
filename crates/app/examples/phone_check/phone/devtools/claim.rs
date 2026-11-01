use std::sync::{Mutex, MutexGuard, PoisonError};

static CLAIMING: Mutex<Claim> = Mutex::new(Claim);

pub struct Claim;

impl Claim {
    pub fn held() -> MutexGuard<'static, Claim> {
        CLAIMING.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

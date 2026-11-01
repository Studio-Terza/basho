use std::time::Instant;

use super::{Chrome, Phone};

pub struct Stopwatch;

impl Stopwatch {
    pub fn timed<Found>(part: &str, work: impl FnOnce(&str) -> Found) -> Found {
        let started = Instant::now();
        let chrome = Chrome::shared();
        let found = work(chrome.endpoint());
        let elapsed = started.elapsed();
        println!(
            "phone check: {part} {elapsed:.2?}, {}",
            Phone::chrome_usage(chrome.endpoint())
        );
        found
    }
}

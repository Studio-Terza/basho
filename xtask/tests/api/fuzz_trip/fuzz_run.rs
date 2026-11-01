use std::env;

use laws::proptest::test_runner::{Config, RngSeed};

pub struct FuzzRun;

impl FuzzRun {
    const CASES_VARIABLE: &'static str = "FUZZ_TRIP_CASES";
    const SEED_VARIABLE: &'static str = "FUZZ_TRIP_SEED";
    const SHORT_RUN: u32 = 32;

    fn number<Number: std::str::FromStr>(name: &str) -> Option<Number> {
        env::var(name).ok().and_then(|text| text.parse().ok())
    }

    pub fn config() -> Config {
        Config {
            cases: Self::number(Self::CASES_VARIABLE).unwrap_or(Self::SHORT_RUN),
            failure_persistence: None,
            rng_seed: RngSeed::Fixed(Self::number(Self::SEED_VARIABLE).unwrap_or(0)),
            ..Config::default()
        }
    }
}

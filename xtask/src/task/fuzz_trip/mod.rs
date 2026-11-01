use crate::task::ci::CargoStep;
use crate::task::failure::Failure;
use crate::task::flag::Flag;
use crate::task::workspace::Workspace;
use newtype::Newtype;

pub struct FuzzTrip {
    cases: String,
    seed: String,
}

impl FuzzTrip {
    const TESTS: [&'static str; 6] = ["test", "-p", "xtask", "--test", "api", "fuzz_trip"];
    const CASES: &'static str = "FUZZ_TRIP_CASES";
    const SEED: &'static str = "FUZZ_TRIP_SEED";
    const LONG_RUN: &'static str = "5000";
    const FIRST_SEED: &'static str = "1";

    pub fn from_flags(flags: impl Iterator<Item = Flag>) -> FuzzTrip {
        let given: Vec<String> = flags.map(Flag::into_inner).collect();
        let nth = |index: usize, fallback: &str| {
            given
                .get(index)
                .map_or_else(|| fallback.to_owned(), Clone::clone)
        };
        FuzzTrip {
            cases: nth(0, Self::LONG_RUN),
            seed: nth(1, Self::FIRST_SEED),
        }
    }

    pub fn campaign(self, workspace: &Workspace) -> Result<(), Failure> {
        CargoStep::new(&Self::TESTS)
            .setting(Self::CASES, self.cases)
            .setting(Self::SEED, self.seed)
            .ran_in(workspace)
    }
}

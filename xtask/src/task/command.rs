use std::collections::BTreeSet;
use std::str::FromStr;

use meta_xtask::Task as Shared;
use strum::VariantNames;

use crate::task::Task;
use crate::task::failure::Failure;
use crate::task::flag::Flag;
use guide::words::Spelling;
use newtype::Newtype;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Basho(Task),
    Shared(Shared),
}

impl Command {
    const SHARED: [Shared; 11] = [
        Shared::Affected,
        Shared::Clippy,
        Shared::MutantCosts,
        Shared::Mutants,
        Shared::MutantsDiffPlan,
        Shared::MutantsFull,
        Shared::MutantsPlan,
        Shared::Stacked,
        Shared::Test,
        Shared::TestRunner,
        Shared::TestTime,
    ];

    #[must_use]
    pub fn usage() -> Failure {
        Failure::Usage(Spelling::new(format!(
            "usage: cargo xtask <{}>",
            Task::VARIANTS
                .iter()
                .copied()
                .chain(Self::SHARED.iter().copied().map(<&str>::from))
                .collect::<BTreeSet<&str>>()
                .into_iter()
                .collect::<Vec<&str>>()
                .join("|")
        )))
    }

    pub fn run(self, flags: impl Iterator<Item = Flag>) -> Result<(), Failure> {
        match self {
            Command::Basho(task) => task.run(flags),
            Command::Shared(task) => task
                .run(flags.map(Flag::into_inner))
                .map_err(Failure::shared),
        }
    }
}

impl FromStr for Command {
    type Err = Failure;

    fn from_str(name: &str) -> Result<Command, Failure> {
        name.parse::<Task>()
            .map(Command::Basho)
            .or_else(|_| {
                name.parse::<Shared>()
                    .ok()
                    .filter(|task| Self::SHARED.contains(task))
                    .map(Command::Shared)
                    .ok_or(())
            })
            .map_err(|()| Command::usage())
    }
}

pub mod policy_budgets;
use std::fs;
use std::time::Instant;

use crate::task::ci::ports::CargoStep;
use crate::task::failure::Failure;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use guide::words::{Label, Spelling};
use policy_budgets::PolicyBudgets;

pub struct Browser;

impl Browser {
    const STEP: &str = "browser";
    const POLICY: &str = "tooling/browser-budget.toml";
    const BUILD: [&str; 7] = [
        "build",
        "--profile",
        "test",
        "--package",
        "basho",
        "--example",
        "phone_check",
    ];
    const CHECK: [&str; 8] = [
        "run",
        "--quiet",
        "--profile",
        "test",
        "--package",
        "basho",
        "--example",
        "phone_check",
    ];
}

impl Browser {
    pub fn check_pages(workspace: &Workspace) -> Result<(), Failure> {
        let policy = workspace.root().join(Self::POLICY);
        let limits = fs::read_to_string(&policy)
            .map_err(|error| Failure::Io {
                site: Site::from(policy.as_path()),
                error,
            })?
            .parse::<PolicyBudgets>()?
            .limits(&Spelling::new(Self::STEP.to_owned()))
            .ok_or(Failure::Unbudgeted(Label::new(Self::STEP)))?;
        CargoStep::new(&Self::BUILD).ran_in(workspace)?;
        let started = Instant::now();
        CargoStep::new(&Self::CHECK).ran_in(workspace)?;
        let elapsed = started.elapsed();
        println!("ci: {} {elapsed:.1?}", Self::STEP);
        limits.judged(Label::new(Self::STEP), elapsed)?;
        limits.past_warning(elapsed).into_iter().for_each(|line| {
            println!(
                "ci: warning: {} took {elapsed:.1?}, past its warning line of {line:.1?}",
                Self::STEP
            );
        });
        Ok(())
    }
}

use std::time::Duration;

use guide::words::{Label, Spelling};
use xtask::task::ci::browser::policy_budgets::PolicyBudgets;

const POLICY: &str = "vague = []\n[budgets.browser]\ndivision = \"performance-and-cost\"\nallowance = { duration = 60000 }\nwindow = \"per_change\"\n[budgets.browser-warning]\ndivision = \"performance-and-cost/browser\"\nallowance = { of = \"browser\", percent = 60 }\nwindow = \"per_change\"\noverspend = \"escalate\"\n";
const STEP: &str = "browser";
const UNDER: Duration = Duration::from_secs(30);
const PAST: Duration = Duration::from_secs(40);
const OVER: Duration = Duration::from_secs(61);

#[test]
fn the_browser_budget_and_its_warning_line_come_from_the_policy() {
    let limits = POLICY
        .parse::<PolicyBudgets>()
        .unwrap()
        .limits(&Spelling::new(STEP.to_owned()))
        .unwrap();
    assert!(limits.past_warning(UNDER).is_none() && limits.past_warning(PAST).is_some());
    assert!(
        limits.judged(Label::new(STEP), PAST).is_ok()
            && limits.judged(Label::new(STEP), OVER).is_err()
    );
    assert!(
        PolicyBudgets::default()
            .limits(&Spelling::new(STEP.to_owned()))
            .is_none()
    );
}

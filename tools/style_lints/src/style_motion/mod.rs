mod motion_problem;

use std::collections::BTreeSet;

use crate::SheetRule;
use crate::style_sheet::{Context, Selector, StyleRule, StyleSheet};
use basho_lints::Report;
use basho_lints::Violation;
use motion_problem::MotionProblem;

pub struct StyleMotion;

impl StyleMotion {
    const MOTION: [&str; 4] = ["transform", "translate", "opacity", "transition"];
    const KEYFRAMES: &str = "@keyframes";
    const ANIMATION: &str = "animation";
    const FOLLOW: &str = "var(--follow)";

    fn problems<'rule>(
        rule: &'rule StyleRule,
        moving: &BTreeSet<&'rule Selector>,
    ) -> Vec<MotionProblem<'rule>> {
        let selector = rule.selector();
        let motion = || {
            rule.declarations()
                .iter()
                .filter(|declaration| Self::MOTION.contains(&&**declaration.property()))
        };
        let keyframes = selector
            .starts_with(Self::KEYFRAMES)
            .then_some(MotionProblem::Keyframes(selector));
        let animated = rule
            .declarations()
            .iter()
            .filter(|declaration| declaration.property().starts_with(Self::ANIMATION))
            .map(|declaration| MotionProblem::Animated(selector, declaration));
        let state = selector
            .classes()
            .into_iter()
            .find(|class| {
                moving
                    .iter()
                    .any(|owner| owner.strip_prefix('.') == Some(class.as_ref()))
            })
            .filter(|_| !moving.contains(selector) && motion().next().is_some())
            .map(|class| MotionProblem::MovedFromOutside(selector, class));
        let literal = motion()
            .filter(|_| moving.contains(selector))
            .filter(|declaration| declaration.assigned().has_literal())
            .map(|declaration| MotionProblem::LiteralPose(selector, declaration));
        let unfollowed = rule
            .declarations()
            .iter()
            .filter(|_| rule.context() == Context::Plain)
            .filter(|declaration| {
                &**declaration.property() == "transition"
                    && !declaration.assigned().contains(Self::FOLLOW)
            })
            .map(|declaration| MotionProblem::Unfollowed(selector, declaration.assigned()));
        keyframes
            .into_iter()
            .chain(animated)
            .chain(state)
            .chain(literal)
            .chain(unfollowed)
            .collect()
    }
}

impl SheetRule for StyleMotion {
    fn report(sheet: &StyleSheet) -> Report {
        let rules = sheet.rules();
        let moving: BTreeSet<&Selector> = rules
            .iter()
            .filter(|rule| {
                rule.declarations()
                    .iter()
                    .any(|declaration| &**declaration.property() == "transition")
            })
            .map(StyleRule::selector)
            .collect();
        Report::new(
            "motion comes from the Rust pose: no keyframes or animations, and a moving element's transform, opacity and transition are set only by its own rule, from pose vars",
            rules
                .iter()
                .flat_map(|rule| {
                    Self::problems(rule, &moving)
                        .into_iter()
                        .map(|problem| Violation::new(rule.line().site(sheet.path()), problem))
                })
                .collect(),
        )
    }
}

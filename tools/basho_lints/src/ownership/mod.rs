mod misplaced_token;

use crate::SourceRule;
use crate::line_number::LineNumber;
use crate::report::Report;
use crate::source_file::{CrateName, SourceFile};
use crate::violation::Violation;
use misplaced_token::MisplacedToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ownership {
    Touch,
    Signal,
}

impl Ownership {
    const ALL: [Ownership; 2] = [Ownership::Touch, Ownership::Signal];

    fn owner(self) -> &'static str {
        match self {
            Ownership::Touch => "crates/app/src/app/screen/touch_surface/",
            Ownership::Signal => "crates/app/src/app/session/",
        }
    }

    fn tokens(self) -> &'static [&'static str] {
        match self {
            Ownership::Touch => &["ontouch"],
            Ownership::Signal => &["use_signal", "Signal<", "Signal::"],
        }
    }

    fn violations(self, file: &SourceFile) -> Vec<Violation> {
        file.text()
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                self.tokens()
                    .iter()
                    .find(|token| line.contains(**token))
                    .map(|token| {
                        Violation::new(
                            LineNumber::from_index(index).site(file.path()),
                            MisplacedToken::new(token, self),
                        )
                    })
            })
            .collect()
    }
}

impl SourceRule for Ownership {
    fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "one owner each: touch handlers live in the touch surface so one place decides who owns a finger; signals live in the session so every state change goes through its methods",
            Self::ALL
                .iter()
                .flat_map(|rule| {
                    files
                        .iter()
                        .filter(|file| file.crate_name() == Some(CrateName::new("app")))
                        .filter(|file| !file.path().starts_with(rule.owner()))
                        .flat_map(|file| rule.violations(file))
                })
                .collect(),
        )
    }
}

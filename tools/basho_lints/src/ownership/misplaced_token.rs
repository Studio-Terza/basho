use std::fmt;

use super::Ownership;

pub struct MisplacedToken {
    token: String,
    rule: Ownership,
}

impl MisplacedToken {
    #[must_use]
    pub fn new(token: impl fmt::Display, rule: Ownership) -> MisplacedToken {
        MisplacedToken {
            token: token.to_string(),
            rule,
        }
    }
}

impl fmt::Display for MisplacedToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} outside {}", self.token, self.rule.owner())
    }
}

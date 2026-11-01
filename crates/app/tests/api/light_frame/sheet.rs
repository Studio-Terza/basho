use super::rule::Rule;
use super::shade::Shade;

const SOURCE: &str = include_str!("../../../assets/main.css");

#[derive(Debug, PartialEq, Eq)]
pub struct Written {
    selector: String,
    value: String,
}

pub struct Sheet {
    rules: Vec<Rule>,
}

impl Sheet {
    pub fn main() -> Sheet {
        Sheet {
            rules: SOURCE.split('}').filter_map(Rule::read).collect(),
        }
    }

    pub fn shade(&self, rule: &str, token: &str) -> Shade {
        self.value(rule, token)
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(|| panic!("{rule} has no shade {token}"))
    }

    pub fn value(&self, rule: &str, token: &str) -> Option<&str> {
        self.rules
            .iter()
            .filter(|candidate| candidate.names(rule))
            .find_map(|candidate| candidate.declares(token))
    }

    pub fn paper_of(&self, selector: &str) -> Option<&str> {
        self.value(selector, "--paper")
    }

    pub fn colours_outside_tokens(&self) -> Vec<Written> {
        self.rules
            .iter()
            .filter(|rule| !rule.is_token_block())
            .flat_map(|rule| {
                rule.values()
                    .filter(|value| {
                        value.contains('#') || value.contains("rgb") || value.contains("hsl")
                    })
                    .map(|value| Written {
                        selector: rule.first_selector().to_owned(),
                        value: value.to_owned(),
                    })
            })
            .collect()
    }
}

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Rule {
    selectors: Vec<String>,
    declarations: BTreeMap<String, String>,
}

impl Rule {
    pub fn read(text: &str) -> Option<Rule> {
        let (selectors, body) = text.rsplit_once('{')?;
        Some(Rule {
            selectors: selectors
                .rsplit_once('\n')
                .map_or(selectors, |(_, last)| last)
                .split(',')
                .map(|selector| selector.trim().to_owned())
                .collect(),
            declarations: body
                .split(';')
                .filter_map(|declaration| declaration.split_once(':'))
                .map(|(name, value)| (name.trim().to_owned(), value.trim().to_owned()))
                .collect(),
        })
    }

    pub fn names(&self, selector: &str) -> bool {
        self.selectors.iter().any(|name| name == selector)
    }

    pub fn declares(&self, name: &str) -> Option<&str> {
        self.declarations.get(name).map(String::as_str)
    }

    pub fn is_token_block(&self) -> bool {
        self.selectors
            .iter()
            .any(|name| name.starts_with(":root") || name.contains(".theme-"))
    }

    pub fn first_selector(&self) -> &str {
        &self.selectors[0]
    }

    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.declarations.values().map(String::as_str)
    }
}

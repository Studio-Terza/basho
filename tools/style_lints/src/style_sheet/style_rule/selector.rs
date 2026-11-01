use std::fmt;

#[derive(derive_more::Deref, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[deref(forward)]
pub struct Selector(String);

#[derive(newtype::Display, Clone, Debug, PartialEq, Eq)]
pub struct CssClass(String);

impl Selector {
    #[must_use]
    pub fn classes(&self) -> Vec<CssClass> {
        self.0
            .split('.')
            .skip(1)
            .filter_map(|rest| rest.split(|character| !Self::in_class(character)).next())
            .filter(|class| !class.is_empty())
            .map(|class| CssClass(class.to_owned()))
            .collect()
    }

    #[must_use]
    pub fn sole_class(&self) -> Option<CssClass> {
        self.0
            .strip_prefix('.')
            .filter(|class| class.chars().all(Self::in_class))
            .map(|class| CssClass(class.to_owned()))
    }

    fn in_class(character: char) -> bool {
        character.is_ascii_alphanumeric() || character == '-' || character == '_'
    }
}

impl From<&str> for Selector {
    fn from(text: &str) -> Selector {
        Selector(text.trim().to_owned())
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

use strum::{Display, EnumIter, IntoEnumIterator, IntoStaticStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumIter, IntoStaticStr)]
pub enum Joint {
    #[strum(serialize = " ")]
    Space,
    #[strum(serialize = "-")]
    Hyphen,
    #[strum(serialize = "")]
    Closed,
}

impl From<&str> for Joint {
    fn from(text: &str) -> Joint {
        Joint::iter()
            .find(|joint| text.starts_with(<&str>::from(joint)))
            .unwrap_or(Joint::Closed)
    }
}

use guide::reading::Reading;
use guide::ruby_text::RubyText;
use strum::{EnumIter, IntoEnumIterator};

use crate::entry::Heading;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Cuisine {
    Soba,
    Ramen,
    Seafood,
    SetMeals,
    Cafes,
    Other,
}

impl Cuisine {
    #[must_use]
    pub fn of_name(name: &RubyText) -> Cuisine {
        let written = name.text(Reading::Written).to_lowercase();
        Cuisine::iter()
            .find(|cuisine| cuisine.words().iter().any(|word| written.contains(word)))
            .unwrap_or(Cuisine::Other)
    }

    #[must_use]
    pub fn heading(self) -> Heading {
        match self {
            Cuisine::Soba => Heading::Soba,
            Cuisine::Ramen => Heading::Ramen,
            Cuisine::Seafood => Heading::Seafood,
            Cuisine::SetMeals => Heading::SetMeals,
            Cuisine::Cafes => Heading::Cafes,
            Cuisine::Other => Heading::OtherFood,
        }
    }

    fn words(self) -> &'static [&'static str] {
        match self {
            Cuisine::Soba => &["soba", "udon"],
            Cuisine::Ramen => &["ramen", "ラーメン", "noodles"],
            Cuisine::Seafood => &["seafood", "tuna", "eel"],
            Cuisine::SetMeals => &["set meals", "shokudo", "kitchen"],
            Cuisine::Cafes => &["cafe", "teahouse"],
            Cuisine::Other => &[],
        }
    }
}

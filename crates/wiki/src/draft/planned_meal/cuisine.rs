use guide::reading::Reading;
use guide::ruby_text::RubyText;
use strum::{EnumIter, IntoEnumIterator};

use crate::entry::Heading;

#[derive(Clone, Copy)]
struct Word(&'static str);

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
            .find(|cuisine| cuisine.words().iter().any(|word| written.contains(word.0)))
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

    fn words(self) -> &'static [Word] {
        match self {
            Cuisine::Soba => &[Word("soba"), Word("udon")],
            Cuisine::Ramen => &[Word("ramen"), Word("ラーメン"), Word("noodles")],
            Cuisine::Seafood => &[Word("seafood"), Word("tuna"), Word("eel")],
            Cuisine::SetMeals => &[Word("set meals"), Word("shokudo"), Word("kitchen")],
            Cuisine::Cafes => &[Word("cafe"), Word("teahouse")],
            Cuisine::Other => &[],
        }
    }
}

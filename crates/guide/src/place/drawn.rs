use newtype::proptest::strategy::{LazyJust, Strategy};

use crate::place::Place;

impl Place {
    const DRAWN: &str = r#"
slug = "eastfield"
name = "Eastfield"
english = "Eastfield"
tagline = ["A made-up village."]
meaning = ["The field to the east."]
excerpts = []
disagreements = []

[origin]
text = ["Made up."]
evidence = { unverified = ["This place is not real."] }

[geography]
text = ["A low ridge between two streams."]
evidence = { unverified = ["This place is not real."] }
"#;

    pub(crate) fn drawn() -> impl Strategy<Value = Place> {
        LazyJust::new(|| toml::from_str(Self::DRAWN).expect("the drawn place is valid"))
    }
}

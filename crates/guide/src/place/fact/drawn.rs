use newtype::proptest::arbitrary::Arbitrary;
use newtype::proptest::strategy::{BoxedStrategy, LazyJust, Strategy};

use super::Fact;

impl Fact {
    const DRAWN: &str = r#"
text = ["A made-up fact."]
evidence = { unverified = ["This fact is not real."] }
"#;
}

impl Arbitrary for Fact {
    type Parameters = ();
    type Strategy = BoxedStrategy<Fact>;

    fn arbitrary_with((): ()) -> BoxedStrategy<Fact> {
        LazyJust::new(|| toml::from_str(Fact::DRAWN).expect("the drawn fact is valid")).boxed()
    }
}

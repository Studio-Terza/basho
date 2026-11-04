#[derive(derive_more::Into, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NameLength(usize);

impl From<&str> for NameLength {
    fn from(text: &str) -> NameLength {
        NameLength(
            text.len()
                - text
                    .trim_start_matches(|character: char| {
                        matches!(character, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
                    })
                    .len(),
        )
    }
}

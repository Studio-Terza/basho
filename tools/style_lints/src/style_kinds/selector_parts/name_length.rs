#[derive(derive_more::Into, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NameLength(usize);

impl From<&str> for NameLength {
    fn from(text: &str) -> NameLength {
        NameLength(
            text.find(|character: char| {
                !(character.is_ascii_alphanumeric() || character == '-' || character == '_')
            })
            .unwrap_or(text.len()),
        )
    }
}

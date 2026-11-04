#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valued {
    Present,
    Matched,
}

impl Valued {
    pub const NAME_END: [char; 7] = ['=', '~', '|', '^', '$', '*', ' '];
}

impl From<&str> for Valued {
    fn from(inner: &str) -> Valued {
        if inner.contains('=') {
            Valued::Matched
        } else {
            Valued::Present
        }
    }
}

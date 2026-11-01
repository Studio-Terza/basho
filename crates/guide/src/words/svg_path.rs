#[derive(newtype::Newtype, newtype::Display, Clone, Debug, PartialEq, Eq)]
pub struct SvgPath(String);

impl SvgPath {
    const CLOSE: &str = "Z";

    #[must_use]
    pub fn of_rings(rings: impl Iterator<Item = SvgPath>) -> SvgPath {
        SvgPath(rings.map(|ring| ring.0 + Self::CLOSE).collect())
    }
}

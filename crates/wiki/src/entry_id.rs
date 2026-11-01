#[derive(newtype::Newtype, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryId(usize);

impl From<EntryId> for usize {
    fn from(id: EntryId) -> usize {
        id.0
    }
}

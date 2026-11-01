use crate::content::Content;

#[derive(newtype::Newtype, derive_more::IntoIterator, Debug, Default)]
pub struct ContentList<'trip>(Vec<&'trip Content>);

impl<'trip> FromIterator<&'trip Content> for ContentList<'trip> {
    fn from_iter<Items: IntoIterator<Item = &'trip Content>>(items: Items) -> Self {
        ContentList(items.into_iter().collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonEmpty<Item> {
    init: Vec<Item>,
    last: Item,
}

impl<Item> NonEmpty<Item> {
    #[must_use]
    pub fn new(mut items: Vec<Item>) -> Option<NonEmpty<Item>> {
        items.pop().map(|last| NonEmpty { init: items, last })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.init.iter().chain(std::iter::once(&self.last))
    }
}

impl<Item> IntoIterator for NonEmpty<Item> {
    type Item = Item;
    type IntoIter = std::iter::Chain<std::vec::IntoIter<Item>, std::iter::Once<Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.init.into_iter().chain(std::iter::once(self.last))
    }
}

impl<Item> From<NonEmpty<Item>> for Vec<Item> {
    fn from(items: NonEmpty<Item>) -> Vec<Item> {
        items.into_iter().collect()
    }
}

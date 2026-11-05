use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dictionary<Name> {
    indexes: BTreeMap<Name, u64>,
}

impl<Name> Default for Dictionary<Name> {
    fn default() -> Dictionary<Name> {
        Dictionary {
            indexes: BTreeMap::new(),
        }
    }
}

impl<Name: Ord + Clone> Dictionary<Name> {
    pub fn index_of(&mut self, name: &Name) -> u64 {
        let next = u64::try_from(self.indexes.len()).unwrap_or(u64::MAX);
        *self.indexes.entry(name.clone()).or_insert(next)
    }

    #[must_use]
    pub fn names_in_order(&self) -> Vec<&Name> {
        let mut ordered: Vec<(&Name, &u64)> = self.indexes.iter().collect();
        ordered.sort_by_key(|entry| *entry.1);
        ordered.into_iter().map(|entry| entry.0).collect()
    }
}

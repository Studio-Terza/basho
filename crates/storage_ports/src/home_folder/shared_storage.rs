use std::sync::Arc;

use crate::Storage;

#[derive(Clone, Debug, derive_more::Deref)]
#[deref(forward)]
pub struct SharedStorage(Arc<dyn Storage>);

impl SharedStorage {
    pub fn new(storage: impl Storage + 'static) -> SharedStorage {
        SharedStorage(Arc::new(storage))
    }
}

impl PartialEq for SharedStorage {
    fn eq(&self, other: &SharedStorage) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for SharedStorage {}

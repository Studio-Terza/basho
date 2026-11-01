use serde::Deserialize;

use crate::connect::identifier::Identifier;

#[derive(Deserialize)]
pub struct Resource<Attributes> {
    id: Identifier,
    attributes: Attributes,
}

impl<Attributes> Resource<Attributes> {
    pub fn identifier(&self) -> &Identifier {
        &self.id
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }
}

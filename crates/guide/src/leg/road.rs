use serde::Deserialize;

use crate::content::Content;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(transparent)]
pub struct Road(Content);

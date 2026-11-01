use serde::Deserialize;

#[derive(Deserialize)]
pub struct Document<Content> {
    data: Content,
}

impl<Content> Document<Content> {
    pub fn into_content(self) -> Content {
        self.data
    }
}

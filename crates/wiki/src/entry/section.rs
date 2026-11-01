use crate::entry::{Heading, Row};

#[derive(Clone, Debug)]
pub struct Section {
    heading: Heading,
    rows: Vec<Row>,
}

impl Section {
    #[must_use]
    pub fn new(heading: Heading, rows: Vec<Row>) -> Section {
        Section { heading, rows }
    }

    #[must_use]
    pub fn heading(&self) -> Heading {
        self.heading
    }

    #[must_use]
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }
}

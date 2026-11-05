use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(from = "Option<Section>")]
pub enum Once<Section> {
    #[default]
    Absent,
    Given(Section),
}

impl<Section> From<Option<Section>> for Once<Section> {
    fn from(section: Option<Section>) -> Once<Section> {
        match section {
            Some(given) => Once::Given(given),
            None => Once::Absent,
        }
    }
}

impl<Section> Once<Section> {
    pub fn joined(self, next: Once<Section>) -> Option<Once<Section>> {
        match self {
            Once::Absent => Some(next),
            Once::Given(section) => next.beside(section),
        }
    }

    fn beside(self, section: Section) -> Option<Once<Section>> {
        match self {
            Once::Absent => Some(Once::Given(section)),
            Once::Given(_) => None,
        }
    }

    pub fn given(self) -> Option<Section> {
        match self {
            Once::Given(section) => Some(section),
            Once::Absent => None,
        }
    }
}

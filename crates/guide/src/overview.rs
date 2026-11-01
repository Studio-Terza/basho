use crate::landmark::Landmarks;

#[derive(Debug, Default)]
pub struct Overview {
    landmarks: Landmarks,
}

impl From<Landmarks> for Overview {
    fn from(landmarks: Landmarks) -> Overview {
        Overview { landmarks }
    }
}

impl Overview {
    #[must_use]
    pub fn landmarks(&self) -> &Landmarks {
        &self.landmarks
    }
}

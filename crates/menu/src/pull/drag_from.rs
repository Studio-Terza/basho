use guide::plane::Pixels;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragFrom {
    Edge,
    Menu,
}

impl DragFrom {
    #[must_use]
    pub fn rest(self, width: Pixels) -> Pixels {
        match self {
            DragFrom::Edge => Pixels::ZERO,
            DragFrom::Menu => width,
        }
    }
}

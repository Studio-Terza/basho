use guide::map_frame::Pixels;
use layout::Touch;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Edge,
    Page,
    Menu,
}

impl Origin {
    #[must_use]
    pub fn on_page(across: Pixels) -> Origin {
        if across < Touch::EdgeZone.extent() {
            Origin::Edge
        } else {
            Origin::Page
        }
    }
}

use dioxus::prelude::*;
use guide::map_frame::Pixels;

#[derive(newtype::Newtype, Clone, Copy, Debug, Default, PartialEq)]
pub struct ScrollTop {
    top: Pixels,
}

impl AsRef<Pixels> for ScrollTop {
    fn as_ref(&self) -> &Pixels {
        &self.top
    }
}

impl From<&Event<MouseData>> for ScrollTop {
    fn from(event: &Event<MouseData>) -> ScrollTop {
        ScrollTop::new(Pixels::new(
            event.data().page_coordinates().y - event.data().client_coordinates().y,
        ))
    }
}

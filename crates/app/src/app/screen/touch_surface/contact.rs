use dioxus::prelude::*;
use guide::map_frame::{Pixels, PlanePoint};

use crate::app::history::ScrollTop;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contact {
    Began,
    Moved,
    Ended,
    Cancelled,
}

impl Contact {
    pub fn top(self, event: &Event<TouchData>) -> Option<ScrollTop> {
        self.touches(event).first().map(|touch| {
            ScrollTop::new(Pixels::new(
                touch.page_coordinates().y - touch.client_coordinates().y,
            ))
        })
    }

    pub fn point(self, event: &Event<TouchData>) -> Option<PlanePoint> {
        self.touches(event).first().map(|touch| {
            let at = touch.client_coordinates();
            PlanePoint::new(Pixels::new(at.x), Pixels::new(at.y))
        })
    }

    fn touches(self, event: &Event<TouchData>) -> Vec<TouchPoint> {
        match self {
            Contact::Began | Contact::Moved => event.data().touches(),
            Contact::Ended => event.data().touches_changed(),
            Contact::Cancelled => Vec::new(),
        }
    }
}

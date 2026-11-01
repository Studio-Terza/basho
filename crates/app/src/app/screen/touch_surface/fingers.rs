use dioxus::prelude::*;
use guide::map_frame::Pixels;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fingers {
    Single,
    Pair,
}

impl Fingers {
    pub fn touching(event: &Event<TouchData>) -> Fingers {
        match event.data().touches().as_slice() {
            [_, _, ..] => Fingers::Pair,
            [] | [_] => Fingers::Single,
        }
    }

    pub fn spread(event: &Event<TouchData>) -> Option<Pixels> {
        match event.data().touches().as_slice() {
            [first, second, ..] => {
                let across = first.client_coordinates().x - second.client_coordinates().x;
                let down = first.client_coordinates().y - second.client_coordinates().y;
                Some(Pixels::new(across.hypot(down)))
            }
            [] | [_] => None,
        }
    }
}

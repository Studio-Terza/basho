use dioxus::prelude::*;
use guide::plane::Pixels;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fingers {
    Single,
    Pair,
}

impl Fingers {
    pub fn touching(event: &Event<TouchData>) -> Fingers {
        event
            .data()
            .touches()
            .get(1)
            .map_or(Fingers::Single, |_| Fingers::Pair)
    }

    pub fn spread(event: &Event<TouchData>) -> Option<Pixels> {
        let touches = event.data().touches();
        touches.first().zip(touches.get(1)).map(|(first, second)| {
            let across = first.client_coordinates().x - second.client_coordinates().x;
            let down = first.client_coordinates().y - second.client_coordinates().y;
            Pixels::new(across.hypot(down))
        })
    }
}

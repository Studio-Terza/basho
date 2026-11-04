mod boundary;

use dioxus::prelude::*;
use guide::plane::{Pixels, PlanePoint};

use super::look::Look;
use boundary::Stamp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shape {
    Square,
    Diamond,
    Cross,
    Bar,
    Star,
    Triangle,
    Dot,
    Target,
    House,
    Ringed,
    Tick,
    Flag,
    Ex,
}

impl Shape {
    pub fn draw(self, at: PlanePoint, size: Pixels, look: Look) -> Element {
        Stamp::drawn(Stamp::new(at, size, look), self)
    }
}

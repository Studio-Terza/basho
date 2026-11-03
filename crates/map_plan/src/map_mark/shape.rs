use guide::classed::Classed;
use std::fmt::Write;

use dioxus::prelude::*;
use guide::plane::Pixels;
use guide::plane::PlanePoint;

use guide::plane::Halving;
use layout::measure::MarkSize;

use super::look::Look;

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
        let across = at[guide::plane::Axis::Across].pixels();
        let down = at[guide::plane::Axis::Down].pixels();
        let half = size.pixels();
        let tone = look.tone();
        let mark_size = look.size();
        match self {
            Shape::Square => {
                rsx! { rect { class: "mark {tone.class()} {mark_size.class()}", x: across - half, y: down - half, width: size.twice().pixels(), height: size.twice().pixels() } }
            }
            Shape::Diamond => {
                let reach = half + MarkSize::outline_reach().pixels();
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across} {down - reach}L{across + reach} {down}L{across} {down + reach}L{across - reach} {down}Z" } }
            }
            Shape::Cross => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across - half} {down}H{across + half}M{across} {down - half}V{down + half}" } }
            }
            Shape::Bar => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across} {down - half}V{down + half}" } }
            }
            Shape::Star => {
                rsx! { polygon { class: "mark {tone.class()} {mark_size.class()}", points: "{Self::star(at, size + MarkSize::outline_reach())}" } }
            }
            Shape::Triangle => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across} {down - half}l{half} {half * (1.0 + MarkSize::PEAK_DROP.fraction())}h{-2.0 * half}z" } }
            }
            Shape::Dot => {
                rsx! { circle { class: "mark {tone.class()} {mark_size.class()}", cx: across, cy: down, r: half } }
            }
            Shape::Target => rsx! {
                circle { class: "mark {tone.class()} {mark_size.class()}", cx: across, cy: down, r: half }
                circle { class: "mark-core", cx: across, cy: down, r: half * MarkSize::CORE.fraction() }
            },
            Shape::House => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across - half} {down + half}v-{half}l{half} -{half}l{half} {half}v{half}z" } }
            }
            Shape::Ringed => rsx! {
                circle { class: "you-ring {tone.class()} {mark_size.class()}", cx: across, cy: down, r: half }
                circle { class: "mark {tone.class()} {mark_size.class()}", cx: across, cy: down, r: half * MarkSize::YOU_DOT.fraction() }
            },
            Shape::Ex => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across - half} {down - half}L{across + half} {down + half}M{across + half} {down - half}L{across - half} {down + half}" } }
            }
            Shape::Tick => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across - half} {down}H{across + half}" } }
            }
            Shape::Flag => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across - half} {down + half}V{down - half}H{across + half}L{across} {down - half * 0.5}H{across - half}z" } }
            }
        }
    }

    fn star(at: PlanePoint, reach: Pixels) -> String {
        let radii = [
            reach.pixels(),
            reach.pixels() * MarkSize::STAR_INNER.fraction(),
        ];
        MarkSize::star_corners()
            .into_iter()
            .zip(radii.into_iter().cycle())
            .fold(String::new(), |mut corners, (angle, radius)| {
                let angle = angle.degrees().to_radians();
                let _ = write!(
                    corners,
                    "{:.1},{:.1} ",
                    at[guide::plane::Axis::Across].pixels() + radius * angle.cos(),
                    at[guide::plane::Axis::Down].pixels() + radius * angle.sin()
                );
                corners
            })
    }
}

use std::fmt::Write;

use dioxus::prelude::*;
use guide::classed::Classed;
use guide::plane::{Axis, Halving, Pixels, PlanePoint};
use layout::measure::MarkSize;

use super::Shape;
use crate::map_mark::look::Look;
use crate::map_mark::tone::Tone;

pub(super) struct Stamp {
    at: PlanePoint,
    size: Pixels,
    across: f64,
    down: f64,
    half: f64,
    reach: f64,
    tone: Tone,
    mark_size: MarkSize,
}

impl Stamp {
    pub(super) fn new(at: PlanePoint, size: Pixels, look: Look) -> Stamp {
        Stamp {
            at,
            size,
            across: at[Axis::Across].pixels(),
            down: at[Axis::Down].pixels(),
            half: size.pixels(),
            reach: size.pixels() + MarkSize::outline_reach().pixels(),
            tone: look.tone(),
            mark_size: look.size(),
        }
    }

    pub(super) fn drawn(
        Stamp {
            at,
            size,
            across,
            down,
            half,
            reach,
            tone,
            mark_size,
        }: Stamp,
        shape: Shape,
    ) -> Element {
        match shape {
            Shape::Square => {
                rsx! { rect { class: "mark {tone.class()} {mark_size.class()}", x: across - half, y: down - half, width: size.twice().pixels(), height: size.twice().pixels() } }
            }
            Shape::Diamond => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across} {down - reach}L{across + reach} {down}L{across} {down + reach}L{across - reach} {down}Z" } }
            }
            Shape::Cross => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across - half} {down}H{across + half}M{across} {down - half}V{down + half}" } }
            }
            Shape::Bar => {
                rsx! { path { class: "mark {tone.class()} {mark_size.class()}", d: "M{across} {down - half}V{down + half}" } }
            }
            Shape::Star => {
                rsx! { polygon { class: "mark {tone.class()} {mark_size.class()}", points: "{PathData::star(at, size)}" } }
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
}

#[derive(newtype::Newtype, newtype::Arbitrary, newtype::Display, Debug)]
pub(super) struct PathData(String);

impl PathData {
    pub(super) fn star(at: PlanePoint, size: Pixels) -> PathData {
        let reach = size + MarkSize::outline_reach();
        let radii = [
            reach.pixels(),
            reach.pixels() * MarkSize::STAR_INNER.fraction(),
        ];
        PathData(
            MarkSize::star_corners()
                .into_iter()
                .zip(radii.into_iter().cycle())
                .fold(String::new(), |mut corners, (angle, radius)| {
                    let angle = angle.degrees().to_radians();
                    let _ = write!(
                        corners,
                        "{:.1},{:.1} ",
                        at[Axis::Across].pixels() + radius * angle.cos(),
                        at[Axis::Down].pixels() + radius * angle.sin()
                    );
                    corners
                }),
        )
    }
}

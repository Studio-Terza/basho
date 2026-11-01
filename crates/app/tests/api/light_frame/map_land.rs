use serde_json::Value;

use super::shade::{Brightness, Contrast, Shade};

const STYLE: &str = include_str!("../../../assets/map_style.json");
const MID_LIGHT: (Brightness, Brightness) = (Brightness::new(0.40), Brightness::new(0.65));
const WATER_APART: Contrast = Contrast::new(1.35);
const LINE_APART: Contrast = Contrast::new(4.5);
const LAND: [&str; 5] = [
    "ground",
    "land-built",
    "land-farm",
    "land-forest",
    "land-green",
];
const DARK_LINES: [&str; 5] = [
    "road-street",
    "road-minor",
    "road-major-sides",
    "route-current",
    "route-later",
];

struct Style(Value);

struct Land {
    id: &'static str,
    shade: Shade,
}

impl Style {
    fn read() -> Style {
        Style(serde_json::from_str(STYLE).unwrap())
    }

    fn shade(&self, id: &str, paint: &str) -> Shade {
        self.0["layers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|layer| layer["id"] == id)
            .and_then(|layer| layer["paint"][paint].as_str())
            .and_then(|text| text.parse().ok())
            .unwrap_or_else(|| panic!("{id} has no {paint}"))
    }

    fn land(&self) -> Vec<Land> {
        LAND.into_iter()
            .map(|id| Land {
                id,
                shade: self.shade(
                    id,
                    if id == "ground" {
                        "background-color"
                    } else {
                        "fill-color"
                    },
                ),
            })
            .collect()
    }
}

#[test]
fn map_land_is_mid_light_grey() {
    let off: Vec<&str> = Style::read()
        .land()
        .into_iter()
        .filter(|land| {
            !land.shade.is_grey() || !(MID_LIGHT.0..=MID_LIGHT.1).contains(&land.shade.brightness())
        })
        .map(|land| land.id)
        .collect();
    assert_eq!(off, Vec::<&str>::new());
}

#[test]
fn water_stays_apart_from_every_land_shade_in_greyscale() {
    let style = Style::read();
    let water = style.shade("water", "fill-color");
    let close: Vec<&str> = style
        .land()
        .into_iter()
        .filter(|land| water.contrast_with(land.shade) < WATER_APART)
        .map(|land| land.id)
        .collect();
    assert_eq!(close, Vec::<&str>::new());
}

#[test]
fn roads_and_the_route_stand_out_from_every_land_shade() {
    let style = Style::read();
    let land = style.land();
    let faint: Vec<&str> = DARK_LINES
        .into_iter()
        .filter(|id| {
            let line = style.shade(id, "line-color");
            land.iter()
                .any(|land| line.contrast_with(land.shade) < LINE_APART)
        })
        .collect();
    assert_eq!(faint, Vec::<&str>::new());
}

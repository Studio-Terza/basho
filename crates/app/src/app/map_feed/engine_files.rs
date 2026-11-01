use std::fmt;

use dioxus::prelude::*;
use manganis::AssetOptions;
use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Clone, Copy, Debug)]
struct EnginePath(Asset);

impl fmt::Display for EnginePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl Serialize for EnginePath {
    fn serialize<Target: Serializer>(&self, target: Target) -> Result<Target::Ok, Target::Error> {
        target.collect_str(self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EngineFiles;

impl EngineFiles {
    const LIBRARY: Asset = asset!(
        "/assets/maplibre-gl.js",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    const WORKER: Asset = asset!(
        "/assets/maplibre-gl-worker.js",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    const SHEET: Asset = asset!(
        "/assets/maplibre-gl.css",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    const STYLE: Asset = asset!(
        "/assets/map_style.json",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    const TILES: &str = "/tiles";
}

impl Serialize for EngineFiles {
    fn serialize<Target: Serializer>(&self, target: Target) -> Result<Target::Ok, Target::Error> {
        let mut files = target.serialize_map(None)?;
        files.serialize_entry("library", &EnginePath(Self::LIBRARY))?;
        files.serialize_entry("worker", &EnginePath(Self::WORKER))?;
        files.serialize_entry("sheet", &EnginePath(Self::SHEET))?;
        files.serialize_entry("style", &EnginePath(Self::STYLE))?;
        files.serialize_entry("tiles", Self::TILES)?;
        files.end()
    }
}

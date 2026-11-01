use dioxus::asset_resolver::asset_path;
#[cfg(target_os = "macos")]
use dioxus::desktop::{DesktopContext, wry::http::Response};
#[cfg(target_os = "ios")]
use dioxus::mobile::{DesktopContext, wry::http::Response};
use dioxus::prelude::*;
use manganis::AssetOptions;
use tiles::{TileArchive, TileId};

pub struct TileServer;

impl TileServer {
    const ARCHIVE: Asset = asset!(
        "/assets/basho.tiles",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    const ROUTE: &str = "tiles";

    pub fn serve() {
        use_hook(|| {
            try_consume_context::<DesktopContext>()
                .iter()
                .for_each(Self::registered_on);
        });
    }

    fn registered_on(desktop: &DesktopContext) {
        let archive = asset_path(Self::ARCHIVE)
            .ok()
            .and_then(|path| TileArchive::open(&path).ok());
        desktop.register_asset_handler(Self::ROUTE.to_owned(), move |request, responder| {
            let name = request
                .uri()
                .path()
                .trim_start_matches('/')
                .trim_start_matches(Self::ROUTE)
                .trim_start_matches('/')
                .parse::<TileId>()
                .ok();
            let tile = archive
                .iter()
                .zip(name)
                .find_map(|(archive, id)| archive.tile(id).ok().flatten());
            responder.respond(Response::new(tile.unwrap_or_default()));
        });
    }
}

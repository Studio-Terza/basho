use map_class::TileLayer;
use map_class::Zoom;

use crate::tile_tag::TileTag;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawnAs {
    layer: TileLayer,
    first_zoom: Zoom,
    tags: Vec<TileTag>,
}

impl DrawnAs {
    #[must_use]
    pub fn new(layer: TileLayer, first_zoom: Zoom, tags: Vec<TileTag>) -> DrawnAs {
        DrawnAs {
            layer,
            first_zoom,
            tags,
        }
    }

    #[must_use]
    pub fn layer(&self) -> TileLayer {
        self.layer
    }

    #[must_use]
    pub fn first_zoom(&self) -> Zoom {
        self.first_zoom
    }

    #[must_use]
    pub fn tags(&self) -> &[TileTag] {
        &self.tags
    }
}

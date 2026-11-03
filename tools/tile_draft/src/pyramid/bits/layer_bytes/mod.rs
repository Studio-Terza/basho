mod dictionary;
mod proto_bytes;

use crate::TileLayer;

use super::GeometryBytes;
use crate::tile_tag::{TagKey, TagText, TileTag};
use dictionary::Dictionary;
use proto_bytes::ProtoBytes;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LayerBytes {
    keys: Dictionary<TagKey>,
    texts: Dictionary<TagText>,
    features: ProtoBytes,
}

impl LayerBytes {
    const VERSION: u64 = 2;
    const EXTENT: u64 = 4096;

    pub fn add(&mut self, tags: &[TileTag], kind: u64, geometry: &GeometryBytes) {
        let packed: Vec<u64> = tags
            .iter()
            .flat_map(|tag| {
                [
                    self.keys.index_of(&tag.key()),
                    self.texts.index_of(tag.text()),
                ]
            })
            .collect();
        let mut feature = ProtoBytes::default();
        feature.bytes_field(2, &ProtoBytes::packed(&packed));
        feature.number_field(3, kind);
        feature.bytes_field(4, &ProtoBytes::packed(geometry.numbers()));
        self.features.bytes_field(2, &feature.into_bytes());
    }

    #[must_use]
    pub fn tile_message(&self, name: TileLayer) -> Vec<u8> {
        let mut layer = ProtoBytes::default();
        layer.number_field(15, Self::VERSION);
        layer.bytes_field(1, <&str>::from(name).as_bytes());
        layer.append(&self.features.clone().into_bytes());
        self.keys.names_in_order().iter().for_each(|key| {
            layer.bytes_field(3, <&str>::from(**key).as_bytes());
        });
        self.texts.names_in_order().iter().for_each(|text| {
            let mut value = ProtoBytes::default();
            value.bytes_field(1, (*text).as_ref());
            layer.bytes_field(4, &value.into_bytes());
        });
        layer.number_field(5, Self::EXTENT);
        let mut tile = ProtoBytes::default();
        tile.bytes_field(3, &layer.into_bytes());
        tile.into_bytes()
    }
}

mod way_node;

use serde::Deserialize;
use serde::de::IgnoredAny;

pub use way_node::WayNode;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MappedWay {
    #[expect(dead_code, reason = "names the OpenStreetMap way in the file")]
    id: IgnoredAny,
    nodes: Vec<WayNode>,
}

impl MappedWay {
    #[must_use]
    pub fn nodes(&self) -> &[WayNode] {
        &self.nodes
    }
}

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum TileLayer {
    Land,
    Water,
    WaterLine,
    Coast,
    Building,
    Road,
    Rail,
    Ferry,
    Terrain,
    Poi,
    Place,
}

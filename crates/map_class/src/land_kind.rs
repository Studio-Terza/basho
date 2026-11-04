use crate::Zoom;

#[derive(strum::IntoStaticStr, strum::EnumString, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LandKind {
    #[strum(to_string = "forest", serialize = "wood", serialize = "scrub")]
    Forest,
    #[strum(
        to_string = "farm",
        serialize = "farmland",
        serialize = "paddy",
        serialize = "orchard",
        serialize = "vineyard",
        serialize = "plant_nursery",
        serialize = "farmyard"
    )]
    Farm,
    #[strum(
        to_string = "built",
        serialize = "residential",
        serialize = "commercial",
        serialize = "industrial",
        serialize = "retail",
        serialize = "railway",
        serialize = "education",
        serialize = "institutional"
    )]
    Built,
    #[strum(
        to_string = "green",
        serialize = "park",
        serialize = "garden",
        serialize = "nature_reserve",
        serialize = "grass",
        serialize = "meadow",
        serialize = "grassland",
        serialize = "heath",
        serialize = "wetland",
        serialize = "cemetery",
        serialize = "beach",
        serialize = "sand"
    )]
    Green,
}

impl From<LandKind> for Zoom {
    fn from(kind: LandKind) -> Zoom {
        match kind {
            LandKind::Forest | LandKind::Farm | LandKind::Built | LandKind::Green => {
                Zoom::LOCAL_ROADS
            }
        }
    }
}

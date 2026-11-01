#[derive(strum::EnumString, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceTag {
    #[strum(
        serialize = "asphalt",
        serialize = "paved",
        serialize = "concrete",
        serialize = "paving_stones"
    )]
    Hard,
    #[strum(
        serialize = "unpaved",
        serialize = "gravel",
        serialize = "dirt",
        serialize = "ground",
        serialize = "compacted",
        serialize = "fine_gravel",
        serialize = "earth",
        serialize = "grass",
        serialize = "mud",
        serialize = "sand",
        serialize = "pebblestone"
    )]
    Loose,
}

use crate::zoom::Zoom;

#[derive(strum::IntoStaticStr, strum::EnumString, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoadRank {
    #[strum(
        to_string = "highway",
        serialize = "motorway",
        serialize = "motorway_link",
        serialize = "trunk",
        serialize = "trunk_link"
    )]
    Highway,
    #[strum(
        to_string = "major",
        serialize = "primary",
        serialize = "primary_link",
        serialize = "secondary",
        serialize = "secondary_link"
    )]
    Major,
    #[strum(
        to_string = "minor",
        serialize = "tertiary",
        serialize = "tertiary_link"
    )]
    Minor,
    #[strum(
        to_string = "street",
        serialize = "unclassified",
        serialize = "residential",
        serialize = "living_street",
        serialize = "road"
    )]
    Street,
    #[strum(to_string = "lane", serialize = "service", serialize = "pedestrian")]
    Lane,
    #[strum(to_string = "track")]
    Track,
    #[strum(
        to_string = "trail",
        serialize = "path",
        serialize = "footway",
        serialize = "steps",
        serialize = "bridleway",
        serialize = "cycleway",
        serialize = "corridor"
    )]
    Trail,
}

impl From<RoadRank> for Zoom {
    fn from(rank: RoadRank) -> Zoom {
        match rank {
            RoadRank::Highway => Zoom::LEAST,
            RoadRank::Major => Zoom::MAJOR_ROADS,
            RoadRank::Minor => Zoom::LOCAL_ROADS,
            RoadRank::Street => Zoom::STREETS,
            RoadRank::Lane | RoadRank::Track | RoadRank::Trail => Zoom::CELL,
        }
    }
}

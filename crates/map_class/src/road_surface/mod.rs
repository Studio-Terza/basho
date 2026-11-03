use crate::road_rank::RoadRank;
mod surface_tag;

pub use surface_tag::SurfaceTag;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum RoadSurface {
    Paved,
    Unpaved,
    Trail,
}

impl RoadSurface {
    #[must_use]
    pub fn of_way(rank: RoadRank, surface: Option<SurfaceTag>) -> RoadSurface {
        match (rank, surface) {
            (RoadRank::Trail, Some(SurfaceTag::Hard)) => RoadSurface::Paved,
            (RoadRank::Trail, _) => RoadSurface::Trail,
            (_, Some(SurfaceTag::Loose)) | (RoadRank::Track, None) => RoadSurface::Unpaved,
            _ => RoadSurface::Paved,
        }
    }
}

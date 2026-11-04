mod land_kind;
mod place_rank;
mod poi_group;

pub use land_kind::LandKind;
pub use place_rank::PlaceRank;
pub use poi_group::PoiGroup;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Zoom(u8);

impl Zoom {
    pub const LEAST: Zoom = Zoom(7);
    pub const RIVERS: Zoom = Zoom(8);
    pub const MAJOR_ROADS: Zoom = Zoom(9);
    pub const LOCAL_ROADS: Zoom = Zoom(10);
    pub const STATIONS: Zoom = Zoom(11);
    pub const STREETS: Zoom = Zoom(12);
    pub const CELL: Zoom = Zoom(13);
    pub const MOST: Zoom = Zoom(14);

    #[must_use]
    pub const fn of_level(level: u8) -> Zoom {
        Zoom(level)
    }

    #[must_use]
    pub const fn level(self) -> u8 {
        self.0
    }

    pub fn up_to_most(self) -> impl Iterator<Item = Zoom> {
        (self.0..=Self::MOST.0).map(Zoom)
    }

    #[must_use]
    pub const fn tiles_across(self) -> u32 {
        1 << self.0
    }

    #[must_use]
    pub const fn zoomed_out(self) -> Zoom {
        Zoom(self.0.saturating_sub(1))
    }
}

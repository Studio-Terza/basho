use crate::Zoom;

#[derive(strum::IntoStaticStr, strum::EnumString, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PoiGroup {
    #[strum(to_string = "konbini", serialize = "convenience")]
    Konbini,
    #[strum(
        to_string = "food",
        serialize = "restaurant",
        serialize = "cafe",
        serialize = "fast_food",
        serialize = "bar",
        serialize = "pub",
        serialize = "food_court",
        serialize = "ice_cream"
    )]
    Food,
    #[strum(to_string = "shop")]
    Shop,
    #[strum(to_string = "worship", serialize = "place_of_worship")]
    Worship,
    #[strum(
        to_string = "sight",
        serialize = "attraction",
        serialize = "viewpoint",
        serialize = "museum",
        serialize = "gallery"
    )]
    Sight,
    #[strum(
        to_string = "lodging",
        serialize = "hotel",
        serialize = "guest_house",
        serialize = "hostel",
        serialize = "motel",
        serialize = "ryokan",
        serialize = "camp_site",
        serialize = "alpine_hut"
    )]
    Lodging,
    #[strum(to_string = "station", serialize = "halt")]
    Station,
    #[strum(
        to_string = "toilet",
        serialize = "toilets",
        serialize = "drinking_water"
    )]
    Toilet,
    #[strum(
        to_string = "service",
        serialize = "post_office",
        serialize = "bank",
        serialize = "atm",
        serialize = "pharmacy",
        serialize = "hospital",
        serialize = "clinic",
        serialize = "police",
        serialize = "fuel",
        serialize = "townhall",
        serialize = "public_bath"
    )]
    Service,
    #[strum(to_string = "peak", serialize = "volcano")]
    Peak,
}

impl From<PoiGroup> for Zoom {
    fn from(group: PoiGroup) -> Zoom {
        match group {
            PoiGroup::Station | PoiGroup::Peak => Zoom::STATIONS,
            PoiGroup::Konbini
            | PoiGroup::Food
            | PoiGroup::Shop
            | PoiGroup::Worship
            | PoiGroup::Sight
            | PoiGroup::Lodging
            | PoiGroup::Toilet
            | PoiGroup::Service => Zoom::MOST,
        }
    }
}

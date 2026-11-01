use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum Heading {
    #[strum(serialize = "The name")]
    TheName,
    Origin,
    Geography,
    #[strum(serialize = "In their words")]
    InTheirWords,
    #[strum(serialize = "Where the accounts differ")]
    AccountsDiffer,
    What,
    Why,
    Way,
    Then,
    Notes,
    #[strum(serialize = "On the way")]
    OnTheWay,
    #[strum(serialize = "On the route")]
    OnTheRoute,
    Day,
    Days,
    Goal,
    Goals,
    Steps,
    #[strum(serialize = "Sights in order")]
    Loop,
    Place,
    Places,
    Inn,
    Inns,
    Evening,
    Evenings,
    Shops,
    Restaurants,
    Meals,
    Things,
    Planned,
    Soba,
    Ramen,
    Seafood,
    #[strum(serialize = "Set meals")]
    SetMeals,
    Cafes,
    #[strum(serialize = "Other food")]
    OtherFood,
    #[strum(serialize = "Food and supplies")]
    Supplies,
    #[strum(serialize = "Souvenirs and crafts")]
    Souvenirs,
    Dinner,
    #[strum(serialize = "At night")]
    AtNight,
    Practical,
    #[strum(serialize = "At the inn")]
    AtTheInn,
    #[strum(serialize = "Linked here")]
    LinkedHere,
}

impl Heading {
    #[must_use]
    pub fn mentioning(self) -> Option<Heading> {
        match self {
            Heading::Day | Heading::Goals | Heading::Steps => None,
            _ => Some(self),
        }
    }
}

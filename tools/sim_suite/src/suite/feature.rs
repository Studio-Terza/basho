use strum::{EnumIter, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, EnumString, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    #[strum(serialize = "TripPickerTests")]
    TripPicker,
    #[strum(serialize = "BugReportTests")]
    BugReport,
    #[strum(serialize = "WikiTests")]
    Wiki,
    #[strum(serialize = "MapTests")]
    Maps,
    #[strum(serialize = "SetNowTests")]
    SetNow,
    #[strum(serialize = "NavigationTests")]
    Navigation,
    #[strum(serialize = "MapPerfTests")]
    MapPerf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrivalMark {
    Arrives,
    AfterSunset,
    AfterCheckIn,
}

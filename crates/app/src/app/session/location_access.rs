use guide::words::Label;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LocationAccess {
    #[default]
    Allowed,
    Asking,
    Denied,
    ServicesOff,
    Approximate,
    Restricted,
}

impl LocationAccess {
    #[must_use]
    pub fn problem(self) -> Option<Label> {
        match self {
            LocationAccess::Allowed => None,
            LocationAccess::Asking => Some(Label::new(
                "LOCATION NOT ALLOWED YET. When the phone asks, tap Allow While Using App.",
            )),
            LocationAccess::Denied => Some(Label::new(
                "LOCATION OFF. Open Settings › Apps › Basho Walk › Location. Set While Using the App. Set Precise Location ON.",
            )),
            LocationAccess::ServicesOff => Some(Label::new(
                "LOCATION SERVICES OFF. Open Settings › Privacy & Security › Location Services. Set it ON.",
            )),
            LocationAccess::Approximate => Some(Label::new(
                "PRECISE LOCATION OFF. Open Settings › Apps › Basho Walk › Location. Set Precise Location ON.",
            )),
            LocationAccess::Restricted => Some(Label::new(
                "LOCATION BLOCKED. Open Settings › Screen Time › Content & Privacy Restrictions › Location Services. Set Allow Changes.",
            )),
        }
    }
}

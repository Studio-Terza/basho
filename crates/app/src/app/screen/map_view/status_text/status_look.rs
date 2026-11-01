use guide::classed::Classed;
use guide::readout::Emphasis;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusLook {
    #[strum(serialize = "status")]
    Plain,
    #[strum(serialize = "status alarm")]
    Alarm,
}

impl StatusLook {
    pub fn for_emphasis(emphasis: Emphasis) -> StatusLook {
        match emphasis {
            Emphasis::Plain => StatusLook::Plain,
            Emphasis::Inverted => StatusLook::Alarm,
        }
    }
}

impl Classed for StatusLook {}

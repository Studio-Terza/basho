use guide::classed::Classed;
#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tone {
    #[strum(serialize = "solid")]
    Solid,
    #[strum(serialize = "hollow")]
    Hollow,
    #[strum(serialize = "stroke")]
    Stroke,
}

impl Classed for Tone {}

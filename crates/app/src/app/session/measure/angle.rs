use guide::Scale;
use guide::geo::{Bearing, Degrees};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Angle {
    CompassSector,
    HeadingDoubt,
    HeadingStep,
}

impl Angle {
    const COMPASS_SECTOR: Degrees = Degrees::new(45.0);
    const SIDES: Scale = Scale::new(2.0);
    const HEADING_STEP: Degrees = Degrees::new(10.0);

    pub fn bearing(self) -> Bearing {
        Bearing::new(match self {
            Angle::CompassSector => Self::COMPASS_SECTOR.degrees(),
            Angle::HeadingDoubt => Angle::CompassSector.bearing().degrees() / Self::SIDES.factor(),
            Angle::HeadingStep => Self::HEADING_STEP.degrees(),
        })
    }
}

use crate::geo::{Coordinate, RouteLine};
use crate::itinerary::Problem;
use crate::way::{MappedSegment, NoMappedPath};

#[derive(Clone, Debug, PartialEq)]
pub enum WalkSegment {
    Mapped(MappedSegment),
    NoMappedPath(NoMappedPath),
}

impl AsRef<RouteLine> for WalkSegment {
    fn as_ref(&self) -> &RouteLine {
        match self {
            WalkSegment::Mapped(segment) => segment.as_ref(),
            WalkSegment::NoMappedPath(segment) => segment.as_ref(),
        }
    }
}

impl WalkSegment {
    pub fn run_into(&self, next: &WalkSegment) -> Result<(), Problem> {
        match self {
            WalkSegment::Mapped(before) => Self::meeting(before, next),
            WalkSegment::NoMappedPath(_) => Ok(()),
        }
    }

    fn meeting(before: &MappedSegment, next: &WalkSegment) -> Result<(), Problem> {
        match next {
            WalkSegment::Mapped(after) => Self::joined(before, after),
            WalkSegment::NoMappedPath(_) => Ok(()),
        }
    }

    fn joined(before: &MappedSegment, after: &MappedSegment) -> Result<(), Problem> {
        before.meets(after)?;
        Ok(())
    }

    #[must_use]
    pub fn no_mapped_path(&self) -> Option<&NoMappedPath> {
        match self {
            WalkSegment::Mapped(_) => None,
            WalkSegment::NoMappedPath(stretch) => Some(stretch),
        }
    }

    #[must_use]
    pub fn between(self, before: Option<&Self>, after: Option<&Self>) -> WalkSegment {
        match self {
            WalkSegment::Mapped(segment) => WalkSegment::Mapped(segment),
            WalkSegment::NoMappedPath(segment) => Self::spanned(&segment, before, after),
        }
    }

    fn spanned(segment: &NoMappedPath, before: Option<&Self>, after: Option<&Self>) -> WalkSegment {
        let line: &RouteLine = segment.as_ref();
        let start =
            before.and_then(|earlier| AsRef::<RouteLine>::as_ref(earlier).points().last().copied());
        let finish =
            after.and_then(|later| AsRef::<RouteLine>::as_ref(later).points().first().copied());
        let points: Vec<Coordinate> = start
            .into_iter()
            .chain(line.points().iter().copied())
            .chain(finish)
            .collect();
        WalkSegment::NoMappedPath(NoMappedPath::spanning(points))
    }
}

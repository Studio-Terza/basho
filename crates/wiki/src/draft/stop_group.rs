use guide::StarPoint;
use guide::geo::{Coordinate, Distance};
use guide::naming::Naming;
use guide::ruby_text::RubyText;

use crate::draft::stop_ref::StopRef;
use crate::entry::EntryKind;
use guide::NameForms;

#[derive(Clone, Debug)]
pub struct StopGroup<'trip> {
    kind: EntryKind,
    name: &'trip RubyText,
    at: Coordinate,
    star: Option<&'trip StarPoint>,
    stops: Vec<StopRef<'trip>>,
}

impl<'trip> StopGroup<'trip> {
    const SAME_SPOT: Distance = Distance::new(200.0);

    #[must_use]
    pub fn of_star(star: &'trip StarPoint) -> StopGroup<'trip> {
        StopGroup {
            kind: EntryKind::Place,
            name: star.text(Naming::Name),
            at: *AsRef::<Coordinate>::as_ref(star),
            star: Some(star),
            stops: Vec::new(),
        }
    }

    #[must_use]
    pub fn group_kind(&self) -> EntryKind {
        self.kind
    }

    #[must_use]
    pub fn group_name(&self) -> &'trip RubyText {
        self.name
    }

    #[must_use]
    pub fn star(&self) -> Option<&'trip StarPoint> {
        self.star
    }

    #[must_use]
    pub fn stops(&self) -> &[StopRef<'trip>] {
        &self.stops
    }

    pub fn with_stop(
        mut groups: Vec<StopGroup<'trip>>,
        stop: StopRef<'trip>,
    ) -> Vec<StopGroup<'trip>> {
        let home = groups.iter().position(|group| group.holds(&stop).is_some());
        let fresh = home
            .is_none()
            .then(|| stop.entry_kind())
            .flatten()
            .map(|kind| StopGroup {
                kind,
                name: stop.stop_name(),
                at: stop.point(),
                star: None,
                stops: Vec::new(),
            });
        groups.extend(fresh);
        let last_index = groups.len().checked_sub(1);
        groups
            .get_mut(home.or(last_index).unwrap_or_default())
            .filter(|group| group.holds(&stop).is_some())
            .into_iter()
            .for_each(|group| group.stops.push(stop));
        groups
    }

    fn holds<'stop>(&self, stop: &'stop StopRef<'trip>) -> Option<&'stop StopRef<'trip>> {
        Some(stop)
            .filter(|stop| stop.entry_kind() == Some(self.kind))
            .filter(|stop| NameForms::head(self.name) == NameForms::head(stop.stop_name()))
            .filter(|stop| self.at.distance_to(stop.point()) <= Self::SAME_SPOT)
    }
}

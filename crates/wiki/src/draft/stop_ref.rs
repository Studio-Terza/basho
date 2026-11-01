use guide::clock::ClockTime;
use guide::content::Content;
use guide::geo::Coordinate;
use guide::itinerary::Day;
use guide::leg::LegIndex;
use guide::leg::along::Along;

use crate::draft::Roster;
use crate::entry::EntryKind;
use crate::entry::{Opening, Row, StopRow};
use crate::{EntryKey, StopIndex};

#[derive(Clone, Copy, Debug)]
pub struct StopRef<'trip> {
    day: &'trip Day,
    leg: LegIndex,
    index: StopIndex,
    stop: &'trip Along,
    name: &'trip Content,
}

impl<'trip> StopRef<'trip> {
    pub fn of_day(day: &'trip Day) -> impl Iterator<Item = StopRef<'trip>> {
        day.legs().iter().enumerate().flat_map(move |(leg, each)| {
            each.along()
                .iter()
                .enumerate()
                .map(move |(position, stop)| StopRef {
                    day,
                    leg: LegIndex::new(leg),
                    index: StopIndex::new(position),
                    stop,
                    name: stop.as_ref(),
                })
        })
    }

    #[must_use]
    pub fn leg_index(&self) -> LegIndex {
        self.leg
    }

    #[must_use]
    pub fn along(&self) -> &'trip Along {
        self.stop
    }

    #[must_use]
    pub fn stop_name(&self) -> &'trip Content {
        self.name
    }

    #[must_use]
    pub fn point(&self) -> Coordinate {
        *AsRef::<Coordinate>::as_ref(self.stop)
    }

    #[must_use]
    pub fn key(&self) -> EntryKey {
        EntryKey::Stop {
            day: self.day.number(),
            leg: self.leg,
            stop: self.index,
        }
    }

    #[must_use]
    pub fn entry_kind(&self) -> Option<EntryKind> {
        EntryKind::of_stop(self.stop.kind())
    }

    #[must_use]
    pub fn row(&self, roster: &Roster<'_>) -> Row {
        let cutoff = self.stop.cutoff();
        let expected = self.stop.expected();
        Row::Stop(
            self.bare(roster)
                .arriving(
                    expected,
                    Opening::judged(expected.map(ClockTime::from), cutoff),
                )
                .told(cutoff.cloned(), self.stop.detail().cloned()),
        )
    }

    #[must_use]
    pub fn evening_row(&self, roster: &Roster<'_>, evening: Option<ClockTime>) -> Row {
        let cutoff = self.stop.cutoff();
        Row::Stop(
            self.bare(roster)
                .arriving(None, Opening::judged(evening, cutoff))
                .told(cutoff.cloned(), self.stop.detail().cloned()),
        )
    }

    fn bare(&self, roster: &Roster<'_>) -> StopRow {
        StopRow::new(
            self.stop_name().clone(),
            roster.find(&self.key()),
            self.day.number(),
        )
    }
}

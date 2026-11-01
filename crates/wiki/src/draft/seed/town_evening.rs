use std::collections::HashSet;

use guide::clock::ClockTime;
use guide::end::End;
use guide::geo::{Coordinate, Distance};
use guide::itinerary::day::Night;
use guide::itinerary::day::goal::{Purpose, Reason};
use guide::itinerary::{Day, Itinerary};
use guide::leg::StopKind;
use guide::reading::Reading;
use guide::words::Spelling;

use crate::EntryKey;
use crate::draft::Roster;
use crate::draft::shelf::Shelf;
use crate::draft::stop_ref::StopRef;
use crate::entry::{Heading, Row, Section};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Want {
    Dinner,
    Sights,
    Practical,
}

impl Want {
    #[must_use]
    pub fn kept<'stop, 'trip>(self, stop: &'stop StopRef<'trip>) -> Option<&'stop StopRef<'trip>> {
        match (self, stop.along().kind()) {
            (Want::Dinner, StopKind::Food)
            | (Want::Sights, StopKind::Sight)
            | (Want::Practical, StopKind::Service) => Some(stop),
            (Want::Practical, StopKind::Shop) => {
                Some(stop).filter(|stop| Shelf::of_stop(stop) == Shelf::Supplies)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TownEvening<'trip> {
    day: &'trip Day,
    night: &'trip Night,
    near: Vec<StopRef<'trip>>,
}

impl<'trip> TownEvening<'trip> {
    const TOWN_WALK: Distance = Distance::new(1500.0);

    #[must_use]
    pub fn new(itinerary: &'trip Itinerary, day: &'trip Day, night: &'trip Night) -> Self {
        let inn = Coordinate::from(night);
        let next = itinerary
            .days()
            .iter()
            .skip_while(|each| each.number() != day.number())
            .nth(1);
        let mut seen: HashSet<Spelling> = HashSet::new();
        let near = std::iter::once(day)
            .chain(next)
            .flat_map(StopRef::of_day)
            .filter(|stop| stop.point().distance_to(inn) <= Self::TOWN_WALK)
            .filter(|stop| seen.insert(stop.stop_name().text(Reading::Written)))
            .collect();
        TownEvening { day, night, near }
    }

    #[must_use]
    pub fn filled(&self) -> Option<&TownEvening<'trip>> {
        (!self.night.notes().is_empty() || !self.near.is_empty()).then_some(self)
    }

    #[must_use]
    pub fn sections(&self, roster: &Roster<'trip>) -> Vec<Section> {
        let number = self.day.number();
        vec![
            Section::new(
                Heading::AtTheInn,
                roster
                    .linked([EntryKey::Inn(number)])
                    .into_iter()
                    .chain(Roster::notes(self.night.notes()))
                    .collect(),
            ),
            Section::new(
                Heading::Dinner,
                self.dinner_goals(roster)
                    .into_iter()
                    .chain(self.rows(roster, Want::Dinner))
                    .collect(),
            ),
            Section::new(Heading::AtNight, self.rows(roster, Want::Sights)),
            Section::new(Heading::Practical, self.rows(roster, Want::Practical)),
            Section::new(Heading::Day, roster.linked([EntryKey::Day(number)])),
        ]
    }

    fn arrival(&self) -> Option<ClockTime> {
        self.day.legs().last().map(|leg| leg.time(End::Finish))
    }

    fn rows(&self, roster: &Roster<'trip>, want: Want) -> Vec<Row> {
        self.near
            .iter()
            .filter_map(|stop| want.kept(stop))
            .map(|stop| stop.evening_row(roster, self.arrival()))
            .collect()
    }

    fn dinner_goals(&self, roster: &Roster<'trip>) -> Vec<Row> {
        let inn = Coordinate::from(self.night);
        roster.linked(
            self.day
                .goals()
                .iter()
                .filter(|goal| matches!(goal.purpose(), Purpose::Aim(Reason::Meal)))
                .filter(|goal| {
                    self.day.leg(goal.leg(End::Finish)).is_some_and(|leg| {
                        leg.point(End::Finish).distance_to(inn) <= Self::TOWN_WALK
                    })
                })
                .map(|goal| EntryKey::Goal {
                    day: self.day.number(),
                    first: goal.leg(End::Start),
                }),
        )
    }
}

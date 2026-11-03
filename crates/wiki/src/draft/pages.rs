use guide::content::Content;
use guide::end::End;
use guide::itinerary::day::Night;
use guide::itinerary::day::goal::Goal;
use guide::itinerary::{Day, DayNumber};
use guide::leg::{Leg, LegIndex};
use guide::leg_line::LegLine;
use guide::place::{Place, PlaceFact};

use crate::EntryKey;
use crate::draft::Roster;
use crate::draft::shelf::Shelf;
use crate::draft::stop_group::StopGroup;
use crate::draft::stop_ref::StopRef;
use crate::entry::{EntryKind, Heading, LoopRow, Row, Section, SourcedFact};

pub struct Pages<'roster, 'trip> {
    roster: &'roster Roster<'trip>,
}

impl<'roster, 'trip> Pages<'roster, 'trip> {
    const SHELVES_AFTER_LEN: usize = 12;
    const LEG_LINES: [LegLine; 6] = [
        LegLine::Way,
        LegLine::Heading,
        LegLine::Handrail,
        LegLine::Then,
        LegLine::Reach,
        LegLine::Why,
    ];

    #[must_use]
    pub fn over(roster: &'roster Roster<'trip>) -> Self {
        Pages { roster }
    }

    #[must_use]
    pub fn roster(&self) -> &'roster Roster<'trip> {
        self.roster
    }

    #[must_use]
    pub fn shops(&self) -> Vec<Section> {
        let groups = self.roster.first_stops(EntryKind::Shop);
        let on_shelf = |wanted: Shelf| {
            Section::new(
                wanted.heading(),
                groups
                    .iter()
                    .filter(|stop| Shelf::of_stop(stop) == wanted)
                    .map(|stop| stop.row(self.roster))
                    .collect(),
            )
        };
        if groups.len() > Self::SHELVES_AFTER_LEN {
            vec![on_shelf(Shelf::Souvenirs), on_shelf(Shelf::Supplies)]
        } else {
            vec![Section::new(
                Heading::Shops,
                groups.iter().map(|stop| stop.row(self.roster)).collect(),
            )]
        }
    }

    #[must_use]
    pub fn day(&self, day: &'trip Day) -> Vec<Section> {
        let number = day.number();
        vec![
            Section::new(
                Heading::Why,
                vec![Row::Text(AsRef::<Content>::as_ref(day).clone())],
            ),
            Section::new(
                Heading::Goals,
                self.roster
                    .linked(day.goals().iter().map(|goal| EntryKey::Goal {
                        day: number,
                        first: goal.leg(End::Start),
                    })),
            ),
            Section::new(Heading::Inn, self.roster.linked([EntryKey::Inn(number)])),
            Section::new(
                Heading::Evening,
                self.roster.linked([EntryKey::Evening(number)]),
            ),
        ]
    }

    #[must_use]
    pub fn goal(&self, day: &'trip Day, goal: &'trip Goal) -> Vec<Section> {
        let number = day.number();
        vec![
            Section::new(
                Heading::Why,
                vec![Row::Text(AsRef::<Content>::as_ref(goal).clone())],
            ),
            Section::new(
                Heading::Loop,
                goal.purpose()
                    .stops()
                    .iter()
                    .map(|stop| Row::Loop(LoopRow::from(stop)))
                    .collect(),
            ),
            Section::new(
                Heading::Places,
                self.roster
                    .linked(goal.purpose().claims().iter().map(|short| EntryKey::Star {
                        day: number,
                        short: short.to_owned(),
                    })),
            ),
            Section::new(
                Heading::Steps,
                self.roster.linked(
                    (usize::from(goal.leg(End::Start))..=usize::from(goal.leg(End::Finish))).map(
                        |leg| EntryKey::Step {
                            day: number,
                            leg: LegIndex::new(leg),
                        },
                    ),
                ),
            ),
            self.day_section(number),
        ]
    }

    #[must_use]
    pub fn step(&self, day: &'trip Day, leg: &'trip Leg) -> Vec<Section> {
        vec![
            Section::new(
                Heading::Way,
                Self::LEG_LINES
                    .into_iter()
                    .map(|line| Row::Text(leg.line_text(line).clone()))
                    .collect(),
            ),
            Section::new(
                Heading::Notes,
                Roster::notes(std::iter::once(leg.see()).chain(leg.notes())),
            ),
            Section::new(
                Heading::Place,
                self.roster
                    .linked(leg.place().map(|slug| EntryKey::Place(slug.clone()))),
            ),
            self.day_section(day.number()),
        ]
    }

    #[must_use]
    pub fn place(place: &'trip Place) -> Vec<Section> {
        let sourced =
            |part: PlaceFact| vec![Row::Sourced(SourcedFact::new(place.fact(part).clone()))];
        vec![
            Section::new(Heading::TheName, vec![Row::Text(place.gloss().clone())]),
            Section::new(Heading::Origin, sourced(PlaceFact::Origin)),
            Section::new(Heading::Geography, sourced(PlaceFact::Geography)),
            Section::new(
                Heading::InTheirWords,
                place.excerpts().iter().cloned().map(Row::Excerpt).collect(),
            ),
            Section::new(
                Heading::AccountsDiffer,
                place
                    .disagreements()
                    .iter()
                    .cloned()
                    .map(Row::Disagreement)
                    .collect(),
            ),
        ]
    }

    #[must_use]
    pub fn inn(&self, day: &'trip Day, night: &'trip Night) -> Vec<Section> {
        let number = day.number();
        vec![
            Section::new(
                Heading::Why,
                vec![
                    Row::CheckIn(night.listed_check_in()),
                    Row::Text(AsRef::<Content>::as_ref(night).clone()),
                ],
            ),
            Section::new(Heading::Notes, Roster::notes(night.notes())),
            Section::new(
                Heading::Evening,
                self.roster.linked([EntryKey::Evening(number)]),
            ),
            self.day_section(number),
        ]
    }

    #[must_use]
    pub fn group(&self, group: &StopGroup<'trip>) -> Vec<Section> {
        let what = group.star().map(|star| Row::Text(star.gloss().clone()));
        let day = group
            .star()
            .map(|star| self.day_section(DayNumber::from(star)));
        [
            Section::new(Heading::What, what.into_iter().collect()),
            Section::new(
                Heading::OnTheRoute,
                group
                    .stops()
                    .iter()
                    .map(|stop: &StopRef<'trip>| stop.row(self.roster))
                    .collect(),
            ),
        ]
        .into_iter()
        .chain(day)
        .collect()
    }

    fn day_section(&self, day: DayNumber) -> Section {
        Section::new(Heading::Day, self.roster.linked([EntryKey::Day(day)]))
    }
}

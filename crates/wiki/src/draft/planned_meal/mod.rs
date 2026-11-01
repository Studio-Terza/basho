use guide::content::Content;
mod cuisine;

use guide::clock::Minutes;
use guide::end::End;
use guide::itinerary::day::goal::{Goal, Purpose, Reason};
use guide::itinerary::{Caption, Day};
use guide::leg::{LegIndex, StopKind};
use guide::place::RubyText;
use guide::reading::Reading;
use strum::IntoEnumIterator;

use crate::EntryKey;
use crate::draft::Roster;
use crate::draft::shelf::Shelf;
use crate::draft::stop_ref::StopRef;
use crate::entry::{Heading, Row, Section};
use cuisine::Cuisine;

#[derive(Clone, Copy, Debug)]
pub enum Choice<'trip> {
    Stop(StopRef<'trip>),
    Goal(&'trip Goal),
}

#[derive(Clone, Debug)]
pub struct PlannedMeal<'trip> {
    day: &'trip Day,
    choice: Choice<'trip>,
    stretch: Vec<StopRef<'trip>>,
}

impl<'trip> PlannedMeal<'trip> {
    const SIT_DOWN: Minutes = Minutes::new(20);

    #[must_use]
    pub fn of_day(day: &'trip Day) -> Vec<PlannedMeal<'trip>> {
        let stops: Vec<StopRef<'trip>> = StopRef::of_day(day).collect();
        let seated: Vec<StopRef<'trip>> = stops
            .iter()
            .filter(|stop| Self::seated(stop))
            .copied()
            .collect();
        let goals = day
            .goals()
            .iter()
            .filter(|goal| matches!(goal.purpose(), Purpose::Aim(Reason::Meal)))
            .filter(|goal| !seated.iter().any(|stop| goal.holds(stop.leg_index())));
        let mut choices: Vec<Choice<'trip>> = seated
            .iter()
            .copied()
            .map(Choice::Stop)
            .chain(goals.map(Choice::Goal))
            .collect();
        choices.sort_by_key(Choice::meal_leg);
        let legs: Vec<LegIndex> = choices.iter().map(Choice::meal_leg).collect();
        choices
            .iter()
            .enumerate()
            .map(|(position, choice)| PlannedMeal {
                day,
                choice: *choice,
                stretch: Self::stretch_around(&stops, &legs, position),
            })
            .collect()
    }

    #[must_use]
    pub fn choice(&self) -> Choice<'trip> {
        self.choice
    }

    #[must_use]
    pub fn sections(&self, roster: &Roster<'trip>) -> Vec<Section> {
        let number = self.day.number();
        let chosen = self.choice.choice_name().text(Reading::Written);
        let of_kind = |kind: StopKind| {
            self.stretch
                .iter()
                .filter(move |stop| stop.along().kind() == kind)
        };
        let others: Vec<&StopRef<'trip>> = of_kind(StopKind::Food)
            .filter(|stop| stop.stop_name().text(Reading::Written) != chosen)
            .collect();
        let by_cuisine = Cuisine::iter().map(|cuisine| {
            Section::new(
                cuisine.heading(),
                others
                    .iter()
                    .filter(|stop| Cuisine::of_name(stop.stop_name()) == cuisine)
                    .map(|stop| stop.row(roster))
                    .collect(),
            )
        });
        std::iter::once(Section::new(
            Heading::Planned,
            self.choice.planned_rows(roster, self.day),
        ))
        .chain(by_cuisine)
        .chain([
            Section::new(
                Heading::Supplies,
                of_kind(StopKind::Shop)
                    .filter(|stop| Shelf::of_stop(stop) == Shelf::Supplies)
                    .map(|stop| stop.row(roster))
                    .collect(),
            ),
            Section::new(Heading::Day, roster.linked([EntryKey::Day(number)])),
        ])
        .collect()
    }

    fn stretch_around(
        stops: &[StopRef<'trip>],
        legs: &[LegIndex],
        meal_index: usize,
    ) -> Vec<StopRef<'trip>> {
        let leg = legs.get(meal_index).copied().unwrap_or_default();
        let halfway = |other: &LegIndex| usize::from(*other).midpoint(usize::from(leg));
        let from_index = meal_index
            .checked_sub(1)
            .and_then(|before| legs.get(before))
            .map(halfway);
        let to_index = legs.get(meal_index + 1).map(halfway);
        stops
            .iter()
            .filter(|stop| {
                let at_index = usize::from(stop.leg_index());
                from_index.is_none_or(|from| at_index > from)
                    && to_index.is_none_or(|to| at_index <= to)
            })
            .copied()
            .collect()
    }

    fn seated(stop: &StopRef<'trip>) -> bool {
        stop.along().kind() == StopKind::Food && stop.along().dwell_minutes() >= Self::SIT_DOWN
    }
}

impl<'trip> Choice<'trip> {
    #[must_use]
    pub fn choice_name(&self) -> &'trip RubyText {
        match self {
            Choice::Stop(stop) => stop.stop_name(),
            Choice::Goal(goal) => &goal[Caption::Name],
        }
    }

    #[must_use]
    pub fn meal_leg(&self) -> LegIndex {
        match self {
            Choice::Stop(stop) => stop.leg_index(),
            Choice::Goal(goal) => goal.leg(End::Finish),
        }
    }

    fn planned_rows(&self, roster: &Roster<'trip>, day: &'trip Day) -> Vec<Row> {
        match self {
            Choice::Stop(stop) => vec![stop.row(roster)],
            Choice::Goal(goal) => roster
                .linked([EntryKey::Goal {
                    day: day.number(),
                    first: goal.leg(End::Start),
                }])
                .into_iter()
                .chain([Row::Text(AsRef::<Content>::as_ref(goal).clone())])
                .collect(),
        }
    }
}

mod pages;
mod town_evening;

use guide::content::Content;
use guide::end::End;
use guide::english_name::EnglishName;
use guide::itinerary::day::Night;
use guide::itinerary::day::goal::{Goal, Purpose};
use guide::itinerary::{Caption, Day, DayNumber, Itinerary};
use guide::landmark::Landmarks;
use guide::leg::{Leg, LegIndex};
use guide::leg_line::LegLine;
use guide::naming::Naming;
use guide::place::{Place, RubyText};
use guide::words::{Label, Spelling};

use crate::draft::Roster;
use crate::draft::planned_meal::PlannedMeal;
use crate::draft::stop_group::StopGroup;
use crate::draft::stop_ref::StopRef;
use crate::entry::{Entry, EntryKind};
use crate::{EntryKey, Home};
use guide::trip::NameForms;
use pages::Pages;
use town_evening::TownEvening;

#[derive(Clone, Debug)]
pub enum Seed<'trip> {
    Index,
    Shops,
    Day(&'trip Day),
    Goal {
        day: &'trip Day,
        goal: &'trip Goal,
    },
    Step {
        day: &'trip Day,
        index: LegIndex,
        leg: &'trip Leg,
    },
    Place(&'trip Place),
    Inn {
        day: &'trip Day,
        night: &'trip Night,
    },
    Evening {
        day: &'trip Day,
        night: &'trip Night,
    },
    Meal(PlannedMeal<'trip>),
    Group(StopGroup<'trip>),
    Feature {
        name: &'trip RubyText,
        english: &'trip EnglishName,
    },
}

impl<'trip> Seed<'trip> {
    #[must_use]
    pub fn of_day(day: &'trip Day) -> Vec<Seed<'trip>> {
        let goals = day.goals().iter().map(|goal| Seed::Goal { day, goal });
        let steps = day
            .legs()
            .iter()
            .enumerate()
            .map(|(position, leg)| Seed::Step {
                day,
                index: LegIndex::new(position),
                leg,
            });
        std::iter::once(Seed::Day(day))
            .chain(goals)
            .chain(steps)
            .collect()
    }

    #[must_use]
    pub fn nights(itinerary: &'trip Itinerary) -> Vec<Seed<'trip>> {
        let days = itinerary.days();
        let inns = days
            .iter()
            .filter_map(|day| day.night().map(|night| Seed::Inn { day, night }));
        let evenings = days.iter().filter_map(|day| {
            day.night()
                .filter(|night| TownEvening::new(itinerary, day, night).filled().is_some())
                .map(|night| Seed::Evening { day, night })
        });
        inns.chain(evenings).collect()
    }

    #[must_use]
    pub fn features(landmarks: &'trip Landmarks) -> Vec<Seed<'trip>> {
        let peaks = landmarks.peaks().iter().map(|peak| Seed::Feature {
            name: peak.as_ref(),
            english: peak.as_ref(),
        });
        let ranges = landmarks.ranges().iter().map(|range| Seed::Feature {
            name: range.as_ref(),
            english: range.as_ref(),
        });
        let seas = landmarks.seas().iter().map(|sea| Seed::Feature {
            name: sea.as_ref(),
            english: sea.as_ref(),
        });
        let rivers = landmarks.rivers().iter().map(|river| Seed::Feature {
            name: river.as_ref(),
            english: river.as_ref(),
        });
        peaks.chain(ranges).chain(seas).chain(rivers).collect()
    }

    #[must_use]
    pub fn entry_kind(&self) -> EntryKind {
        match self {
            Seed::Index | Seed::Shops => EntryKind::Guide,
            Seed::Day(_) => EntryKind::Day,
            Seed::Goal { .. } => EntryKind::Goal,
            Seed::Step { .. } => EntryKind::Step,
            Seed::Place(_) => EntryKind::Place,
            Seed::Inn { .. } => EntryKind::Inn,
            Seed::Evening { .. } => EntryKind::Evening,
            Seed::Meal(_) => EntryKind::PlannedMeal,
            Seed::Group(group) => group.group_kind(),
            Seed::Feature { .. } => EntryKind::Thing,
        }
    }

    #[must_use]
    pub fn home(&self) -> Option<Home> {
        match self {
            Seed::Day(day) => Some(Home::Day(day.number())),
            Seed::Goal { day, goal } => Some(Home::Goal {
                day: day.number(),
                first: goal.leg(End::Start),
            }),
            Seed::Step { day, index, .. } => Some(Home::Step {
                day: day.number(),
                leg: *index,
            }),
            Seed::Index
            | Seed::Shops
            | Seed::Place(_)
            | Seed::Inn { .. }
            | Seed::Evening { .. }
            | Seed::Meal(_)
            | Seed::Group(_)
            | Seed::Feature { .. } => None,
        }
    }

    #[must_use]
    pub fn keys(&self) -> Vec<EntryKey> {
        match self {
            Seed::Index => vec![EntryKey::Index],
            Seed::Shops => vec![EntryKey::Shops],
            Seed::Day(day) => vec![EntryKey::Day(day.number())],
            Seed::Goal { day, goal } => vec![EntryKey::Goal {
                day: day.number(),
                first: goal.leg(End::Start),
            }],
            Seed::Step { day, index, .. } => vec![EntryKey::Step {
                day: day.number(),
                leg: *index,
            }],
            Seed::Place(place) => vec![EntryKey::Place(place.slug().to_owned())],
            Seed::Inn { day, .. } => vec![EntryKey::Inn(day.number())],
            Seed::Evening { day, .. } => vec![EntryKey::Evening(day.number())],
            Seed::Group(group) => group
                .stops()
                .iter()
                .map(StopRef::key)
                .chain(group.star().map(|star| EntryKey::Star {
                    day: DayNumber::from(star),
                    short: star.short().to_owned(),
                }))
                .collect(),
            Seed::Meal(_) | Seed::Feature { .. } => Vec::new(),
        }
    }

    #[must_use]
    pub fn first_stop(&self, kind: EntryKind) -> Option<StopRef<'trip>> {
        match self {
            Seed::Group(group) if group.group_kind() == kind => group.stops().first().copied(),
            _ => None,
        }
    }

    #[must_use]
    pub fn names(&self) -> Vec<Spelling> {
        match self {
            Seed::Place(place) => {
                NameForms::of_name(place.text(Naming::Name)).also(place.english())
            }
            Seed::Inn { night, .. } => {
                NameForms::of_name(&night[Caption::Name]).also(AsRef::as_ref(*night))
            }
            Seed::Goal { goal, .. } if matches!(goal.purpose(), Purpose::Aim(_)) => goal
                .purpose()
                .stops()
                .iter()
                .flat_map(|stop| AsRef::<Content>::as_ref(stop).mentions())
                .map(NameForms::of_name)
                .fold(NameForms::of_name(&goal[Caption::Name]), NameForms::joined),
            Seed::Group(group) => group
                .star()
                .into_iter()
                .fold(NameForms::of_name(group.group_name()), |forms, star| {
                    forms.also(star.english()).also(&star.short_english())
                }),
            Seed::Feature { name, english } => NameForms::of_name(name).also(english),
            Seed::Day(day) if day.number() > DayNumber::default() => {
                NameForms::of_day(day.number())
            }
            Seed::Index
            | Seed::Shops
            | Seed::Day(_)
            | Seed::Goal { .. }
            | Seed::Step { .. }
            | Seed::Evening { .. }
            | Seed::Meal(_) => NameForms::default(),
        }
        .into_forms()
    }

    #[must_use]
    pub fn entry(&self, roster: &Roster<'trip>) -> Entry {
        let pages = Pages::over(roster);
        let entry = Entry::new(self.entry_kind(), self.entry_name()).homed(self.home());
        match self {
            Seed::Index => entry,
            Seed::Shops => entry.with_sections(pages.shops()),
            Seed::Day(day) => entry.with_sections(pages.day(day)),
            Seed::Goal { day, goal } => entry.with_sections(pages.goal(day, goal)),
            Seed::Step { day, leg, .. } => entry.with_sections(pages.step(day, leg)),
            Seed::Place(place) => entry
                .titled(
                    Some(place.english().clone()),
                    Some(AsRef::<guide::content::Content>::as_ref(*place).clone()),
                )
                .with_sections(Pages::place(place)),
            Seed::Inn { day, night } => entry
                .titled(Some(AsRef::<EnglishName>::as_ref(*night).clone()), None)
                .backed_by(night.as_ref())
                .with_sections(pages.inn(day, night)),
            Seed::Evening { day, night } => entry
                .titled(Some(AsRef::<EnglishName>::as_ref(*night).clone()), None)
                .backed_by(night.as_ref())
                .with_sections(TownEvening::new(roster.itinerary(), day, night).sections(roster)),
            Seed::Meal(meal) => entry.with_sections(meal.sections(roster)),
            Seed::Group(group) => entry
                .titled(group.star().map(|star| star.english().clone()), None)
                .with_sections(pages.group(group)),
            Seed::Feature { english, .. } => entry.titled(Some((*english).clone()), None),
        }
    }

    fn entry_name(&self) -> RubyText {
        match self {
            Seed::Index => RubyText::from(Label::new("Index")),
            Seed::Shops => RubyText::from(Label::new("Shops")),
            Seed::Day(day) => day[Caption::Name].clone(),
            Seed::Goal { goal, .. } => goal[Caption::Name].clone(),
            Seed::Step { leg, .. } => leg.line_text(LegLine::Way).shown().clone(),
            Seed::Place(place) => place.text(Naming::Name).clone(),
            Seed::Inn { night, .. } | Seed::Evening { night, .. } => night[Caption::Name].clone(),
            Seed::Meal(meal) => meal.choice().choice_name().clone(),
            Seed::Group(group) => group.group_name().clone(),
            Seed::Feature { name, .. } => (*name).clone(),
        }
    }
}

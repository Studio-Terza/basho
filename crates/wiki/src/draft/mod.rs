mod planned_meal;
mod seed;
mod shelf;
mod stop_group;
mod stop_ref;

use std::collections::{BTreeMap, HashMap, HashSet};

use guide::itinerary::{Day, Itinerary};
use guide::landmark::Landmarks;
use guide::place::{Fact, NoteKind, Place};
use guide::words::Spelling;

use crate::entry::EntryKind;
use crate::entry::{Entry, Row};
use crate::name_table::Claim;
use crate::{EntryId, EntryKey};
use planned_meal::PlannedMeal;
use seed::Seed;
use stop_group::StopGroup;
use stop_ref::StopRef;

pub struct Roster<'trip> {
    itinerary: &'trip Itinerary,
    seeds: Vec<Seed<'trip>>,
    keys: HashMap<EntryKey, EntryId>,
}

impl<'trip> Roster<'trip> {
    #[must_use]
    pub fn new(
        itinerary: &'trip Itinerary,
        places: &'trip [Place],
        landmarks: &'trip Landmarks,
    ) -> Roster<'trip> {
        let days = itinerary.days();
        let named: Vec<Seed<'trip>> = [Seed::Index, Seed::Shops]
            .into_iter()
            .chain(days.iter().flat_map(Seed::of_day))
            .chain(places.iter().map(Seed::Place))
            .chain(Seed::nights(itinerary))
            .chain(days.iter().flat_map(PlannedMeal::of_day).map(Seed::Meal))
            .chain(Self::groups(days, landmarks))
            .collect();
        let taken: HashSet<Spelling> = named.iter().flat_map(Seed::names).collect();
        let seeds: Vec<Seed<'trip>> = named
            .into_iter()
            .chain(
                Seed::features(landmarks)
                    .into_iter()
                    .filter(|feature| feature.names().iter().all(|name| !taken.contains(name))),
            )
            .collect();
        let keys = seeds
            .iter()
            .enumerate()
            .flat_map(|(position, seed)| {
                seed.keys()
                    .into_iter()
                    .map(move |key| (key, EntryId::new(position)))
            })
            .collect();
        Roster {
            itinerary,
            seeds,
            keys,
        }
    }

    #[must_use]
    pub fn find(&self, key: &EntryKey) -> Option<EntryId> {
        self.keys.get(key).copied()
    }

    #[must_use]
    pub fn linked(&self, keys: impl IntoIterator<Item = EntryKey>) -> Vec<Row> {
        keys.into_iter()
            .filter_map(|key| self.find(&key))
            .map(Row::Entry)
            .collect()
    }

    pub fn notes(facts: impl IntoIterator<Item = &'trip Fact>) -> Vec<Row> {
        facts
            .into_iter()
            .filter(|fact| fact.kind() != NoteKind::Bookkeeping)
            .cloned()
            .map(Row::Note)
            .collect()
    }

    #[must_use]
    pub fn itinerary(&self) -> &'trip Itinerary {
        self.itinerary
    }

    #[must_use]
    pub fn first_stops(&self, kind: EntryKind) -> Vec<StopRef<'trip>> {
        self.seeds
            .iter()
            .filter_map(|seed| seed.first_stop(kind))
            .collect()
    }

    #[must_use]
    pub fn entries(&self) -> Vec<Entry> {
        self.seeds.iter().map(|seed| seed.entry(self)).collect()
    }

    #[must_use]
    pub fn into_keys(self) -> HashMap<EntryKey, EntryId> {
        self.keys
    }

    #[must_use]
    pub fn claims(&self) -> BTreeMap<Spelling, Claim> {
        let mut claims: BTreeMap<Spelling, Claim> = BTreeMap::new();
        self.seeds
            .iter()
            .enumerate()
            .flat_map(|(position, seed)| {
                let claim = Claim::new(EntryId::new(position), seed.entry_kind());
                seed.names()
                    .into_iter()
                    .map(move |name| Named { name, claim })
            })
            .for_each(|named| {
                claims
                    .entry(named.name)
                    .and_modify(|held| *held = held.joined(named.claim))
                    .or_insert(named.claim);
            });
        claims
    }

    fn groups(days: &'trip [Day], landmarks: &'trip Landmarks) -> Vec<Seed<'trip>> {
        let stars = landmarks
            .star_points()
            .iter()
            .map(StopGroup::of_star)
            .collect();
        StopGroup::gathered(stars, days.iter().flat_map(StopRef::of_day))
            .into_iter()
            .map(Seed::Group)
            .collect()
    }
}

struct Named {
    name: Spelling,
    claim: Claim,
}

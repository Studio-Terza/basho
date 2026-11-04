mod claim;
mod draft;
pub mod entry;
mod entry_id;
mod entry_key;
mod home;
mod name_table;

use std::collections::{BTreeMap, BTreeSet, HashMap};

use guide::itinerary::Itinerary;
use guide::landmark::Landmarks;
use guide::place::Place;
use strum::IntoEnumIterator;

use draft::Roster;
use entry::{Entry, EntryKind, Heading, Row, Section};
use name_table::NameTable;

pub use entry_id::EntryId;
pub use entry_key::{EntryKey, StopIndex};
pub use home::Home;

#[derive(Debug, Default)]
pub struct Wiki {
    entries: Vec<Entry>,
    keys: HashMap<EntryKey, EntryId>,
    names: NameTable,
}

impl Wiki {
    #[must_use]
    pub fn new(itinerary: &Itinerary, places: &[Place], landmarks: &Landmarks) -> Wiki {
        let roster = Roster::new(itinerary, places, landmarks);
        let names = NameTable::new(&roster.claims());
        let drafted = roster.entries();
        let linked = Self::backlinks(&drafted, &names);
        let index = roster.find(&EntryKey::Index);
        let listed = Self::index(&drafted);
        let entries = drafted
            .into_iter()
            .enumerate()
            .map(|(position, entry)| {
                let id = EntryId::new(position);
                entry.with_sections(
                    index
                        .filter(|home| *home == id)
                        .map_or_else(|| Self::linked_here(linked.get(&id)), |_| listed.clone()),
                )
            })
            .collect();
        Wiki {
            entries,
            keys: roster.into_keys(),
            names,
        }
    }

    #[must_use]
    pub fn find(&self, key: &EntryKey) -> Option<EntryId> {
        self.keys.get(key).copied()
    }

    #[must_use]
    pub fn entry(&self, id: EntryId) -> Option<&Entry> {
        self.entries.get(usize::from(id))
    }

    #[must_use]
    pub fn pages(&self) -> Vec<EntryId> {
        (0..self.entries.len())
            .map(EntryId::new)
            .filter(|id| self.entry(*id).is_some_and(|entry| entry.home().is_none()))
            .collect()
    }

    #[must_use]
    pub fn names(&self) -> &NameTable {
        &self.names
    }

    fn linked_here(sources: Option<&BTreeSet<EntryId>>) -> Vec<Section> {
        sources
            .map(|sources| {
                Section::new(
                    Heading::LinkedHere,
                    sources.iter().copied().map(Row::Entry).collect(),
                )
            })
            .into_iter()
            .collect()
    }

    fn backlinks(entries: &[Entry], names: &NameTable) -> BTreeMap<EntryId, BTreeSet<EntryId>> {
        let mut linked: BTreeMap<EntryId, BTreeSet<EntryId>> = BTreeMap::new();
        entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.kind() != EntryKind::Guide)
            .for_each(|(position, entry)| {
                let source = EntryId::new(position);
                names
                    .links_of(entry)
                    .into_iter()
                    .filter(|target| *target != source)
                    .for_each(|target| {
                        linked.entry(target).or_default().insert(source);
                    });
            });
        linked
    }

    fn index(entries: &[Entry]) -> Vec<Section> {
        EntryKind::iter()
            .filter_map(|kind| kind.index_heading().map(|heading| (kind, heading)))
            .map(|(kind, heading)| Section::new(heading, Self::of_kind(entries, kind)))
            .collect()
    }

    fn of_kind(entries: &[Entry], kind: EntryKind) -> Vec<Row> {
        entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.kind() == kind)
            .map(|(position, _)| Row::Entry(EntryId::new(position)))
            .collect()
    }
}

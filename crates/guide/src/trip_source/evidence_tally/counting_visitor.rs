use std::fmt;
use std::iter;

use serde::de::{Error, MapAccess, SeqAccess, Visitor};

use crate::EvidenceCounts;
use crate::evidence::Evidence;
use crate::trip_source::EvidenceTally;
use crate::words::Spelling;

pub struct CountingVisitor;

impl CountingVisitor {
    const KEY: &str = "evidence";

    fn next_entry<'de, Entries: MapAccess<'de>>(
        entries: &mut Entries,
    ) -> Result<Option<EvidenceCounts>, Entries::Error> {
        entries
            .next_key::<Spelling>()?
            .map(|key| Self::counts_under(&key, entries))
            .transpose()
    }

    fn counts_under<'de, Entries: MapAccess<'de>>(
        key: &Spelling,
        entries: &mut Entries,
    ) -> Result<EvidenceCounts, Entries::Error> {
        match EntryKind::from(key) {
            EntryKind::Evidence => entries
                .next_value::<Evidence>()
                .map(|evidence| EvidenceCounts::from(evidence.kind())),
            EntryKind::Section => entries
                .next_value::<EvidenceTally>()
                .map(EvidenceCounts::from),
        }
    }
}

enum EntryKind {
    Evidence,
    Section,
}

impl From<&Spelling> for EntryKind {
    fn from(key: &Spelling) -> EntryKind {
        Some(key)
            .filter(|key| key.as_ref() == CountingVisitor::KEY)
            .map_or(EntryKind::Section, |_| EntryKind::Evidence)
    }
}

impl<'de> Visitor<'de> for CountingVisitor {
    type Value = EvidenceCounts;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any part of a trip file")
    }

    fn visit_bool<Failure: Error>(self, _: bool) -> Result<EvidenceCounts, Failure> {
        Ok(EvidenceCounts::default())
    }

    fn visit_i64<Failure: Error>(self, _: i64) -> Result<EvidenceCounts, Failure> {
        Ok(EvidenceCounts::default())
    }

    fn visit_f64<Failure: Error>(self, _: f64) -> Result<EvidenceCounts, Failure> {
        Ok(EvidenceCounts::default())
    }

    fn visit_str<Failure: Error>(self, _: &str) -> Result<EvidenceCounts, Failure> {
        Ok(EvidenceCounts::default())
    }

    fn visit_seq<Items: SeqAccess<'de>>(
        self,
        mut items: Items,
    ) -> Result<EvidenceCounts, Items::Error> {
        iter::from_fn(|| items.next_element::<EvidenceTally>().transpose())
            .map(|found| found.map(EvidenceCounts::from))
            .sum()
    }

    fn visit_map<Entries: MapAccess<'de>>(
        self,
        mut entries: Entries,
    ) -> Result<EvidenceCounts, Entries::Error> {
        iter::from_fn(|| Self::next_entry(&mut entries).transpose()).sum()
    }
}

use guide::content::Content;
use guide::place::disagreement::Disagreement;
use guide::place::{Excerpt, ExcerptText, Fact};
use guide::times::ListedTime;

use crate::EntryId;
use crate::entry::{LoopRow, StopRow};

#[derive(newtype::Newtype, newtype::Arbitrary, Clone, Debug)]
pub struct SourcedFact {
    fact: Fact,
}

impl AsRef<Fact> for SourcedFact {
    fn as_ref(&self) -> &Fact {
        &self.fact
    }
}

#[derive(Clone, Debug)]
pub enum Row {
    Text(Content),
    Note(Fact),
    Sourced(SourcedFact),
    CheckIn(ListedTime),
    Entry(EntryId),
    Stop(StopRow),
    Loop(LoopRow),
    Excerpt(Excerpt),
    Disagreement(Disagreement),
}

impl Row {
    #[must_use]
    pub fn text(&self) -> Option<&Content> {
        match self {
            Row::Text(text) => Some(text),
            _ => None,
        }
    }

    #[must_use]
    pub fn note(&self) -> Option<&Fact> {
        match self {
            Row::Note(fact) => Some(fact),
            _ => None,
        }
    }

    #[must_use]
    pub fn sourced(&self) -> Option<&SourcedFact> {
        match self {
            Row::Sourced(fact) => Some(fact),
            _ => None,
        }
    }

    #[must_use]
    pub fn check_in(&self) -> Option<ListedTime> {
        match self {
            Row::CheckIn(time) => Some(*time),
            _ => None,
        }
    }

    #[must_use]
    pub fn entry(&self) -> Option<EntryId> {
        match self {
            Row::Entry(entry) => Some(*entry),
            _ => None,
        }
    }

    #[must_use]
    pub fn stop(&self) -> Option<&StopRow> {
        match self {
            Row::Stop(stop) => Some(stop),
            _ => None,
        }
    }

    #[must_use]
    pub fn looped(&self) -> Option<&LoopRow> {
        match self {
            Row::Loop(stop) => Some(stop),
            _ => None,
        }
    }

    #[must_use]
    pub fn excerpt(&self) -> Option<&Excerpt> {
        match self {
            Row::Excerpt(excerpt) => Some(excerpt),
            _ => None,
        }
    }

    #[must_use]
    pub fn disagreement(&self) -> Option<&Disagreement> {
        match self {
            Row::Disagreement(disagreement) => Some(disagreement),
            _ => None,
        }
    }

    #[must_use]
    pub fn links(&self) -> Vec<EntryId> {
        self.entry()
            .into_iter()
            .chain(self.stop().and_then(StopRow::stop_entry))
            .collect()
    }

    #[must_use]
    pub fn texts(&self) -> Vec<&Content> {
        let fact = self
            .note()
            .or_else(|| self.sourced().map(AsRef::as_ref))
            .map(Fact::text);
        let stop = self.stop().into_iter().flat_map(|stop| {
            std::iter::once(stop.stop_name()).chain(stop.detail().map(Fact::text))
        });
        let looped = self.looped().map(LoopRow::name);
        let quoted = self
            .excerpt()
            .map(|excerpt| excerpt.text(ExcerptText::Translation));
        let disputed = self.disagreement().into_iter().flat_map(|disagreement| {
            [
                AsRef::<Content>::as_ref(disagreement),
                disagreement.detail(),
            ]
        });
        self.text()
            .into_iter()
            .chain(fact)
            .chain(stop)
            .chain(looped)
            .chain(quoted)
            .chain(disputed)
            .collect()
    }
}

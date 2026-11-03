mod draft_name;
mod draft_part;
mod name_forms;
mod name_set;

use crate::content::{Content, NameKind, Part};
use crate::english_name::EnglishName;
use crate::itinerary::{Caption, Day, Finding, Itinerary, Problem};
use crate::landmark::Landmarks;
use crate::leg::{Leg, StopKind};
use crate::naming::Naming;
use crate::place::Place;
use crate::reading::Reading;
use crate::readout::DayCatch;
use crate::ruby_text::RubyText;
use crate::words::Spelling;
use draft_name::DraftName;
use name_set::NameSet;

pub use draft_part::DraftPart;
pub use name_forms::NameForms;

#[derive(Debug)]
pub struct KnownNames {
    entries: NameSet,
    spots: NameSet,
}

impl KnownNames {
    #[must_use]
    pub fn new(itinerary: &Itinerary, places: &[Place], landmarks: &Landmarks) -> KnownNames {
        let days = itinerary.days();
        let entries = places
            .iter()
            .flat_map(|place| {
                NameForms::of_name(place.text(Naming::Name))
                    .also(place.english())
                    .into_forms()
            })
            .chain(days.iter().flat_map(Self::entries_of_day))
            .chain(landmarks.star_points().iter().flat_map(|star| {
                NameForms::of_name(star.text(Naming::Name))
                    .also(star.english())
                    .also(&star.short_english())
                    .into_forms()
            }))
            .chain(NameForms::of_features(landmarks))
            .collect();
        KnownNames {
            entries: NameSet::new(entries),
            spots: NameSet::new(Self::spots(days)),
        }
    }

    fn entries_of_day(day: &Day) -> Vec<Spelling> {
        let inn = day.night().into_iter().flat_map(|night| {
            NameForms::of_name(&night[Caption::Name])
                .also(AsRef::<EnglishName>::as_ref(night))
                .into_forms()
        });
        let aims = day.aims().iter().flat_map(|aim| {
            aim.reason()
                .sight_names()
                .flat_map(Content::mentions)
                .chain(std::iter::once(&aim[Caption::Name]))
                .flat_map(|name| NameForms::of_name(name).into_forms())
        });
        let stops = day
            .legs()
            .iter()
            .flat_map(Leg::along)
            .filter(|stop| {
                matches!(
                    stop.kind(),
                    StopKind::Shop | StopKind::Food | StopKind::Sight
                )
            })
            .flat_map(|stop| NameForms::of_name(stop.as_ref()).into_forms());
        inn.chain(aims)
            .chain(stops)
            .chain(NameForms::of_day(day.number()).into_forms())
            .collect()
    }

    fn spots(days: &[Day]) -> Vec<Spelling> {
        let legs = || days.iter().flat_map(Day::legs);
        let ends = legs().filter_map(|leg| leg.alighting());
        let stops = legs()
            .flat_map(Leg::along)
            .filter(|stop| matches!(stop.kind(), StopKind::Landmark | StopKind::Service))
            .map(AsRef::<RubyText>::as_ref);
        ends.chain(stops)
            .map(|name| name.text(Reading::Written))
            .chain(
                days.iter()
                    .filter_map(Day::catch)
                    .flat_map(DayCatch::spot_names),
            )
            .collect()
    }

    fn dangling(&self, part: &Part) -> Option<Problem> {
        let unknown = |kind: NameKind, names: &NameSet| {
            part.named(kind)
                .map(|name| name.text(Reading::Written))
                .filter(|name| !names.holds(name))
        };
        unknown(NameKind::Entry, &self.entries)
            .map(|name| Problem::UnknownEntry { name })
            .or_else(|| {
                unknown(NameKind::Spot, &self.spots).map(|name| Problem::UnknownSpot { name })
            })
    }

    fn named_at(&self, note: &RubyText, at_index: usize) -> Option<DraftPart> {
        let named = |kind: NameKind, length: usize| {
            DraftPart::named(kind, &note.slice(at_index, at_index + length))
        };
        self.entries
            .length_at(note, at_index)
            .map(|length| named(NameKind::Entry, length))
            .or_else(|| {
                self.spots
                    .length_at(note, at_index)
                    .map(|length| named(NameKind::Spot, length))
            })
    }

    #[must_use]
    pub fn drafted(&self, note: &RubyText) -> Vec<DraftPart> {
        let written = note.letters();
        (0..written.len())
            .fold(DraftName::default(), |cut, at_index| {
                cut.past(note, at_index, self.named_at(note, at_index))
            })
            .finished(note, written.len())
    }

    #[must_use]
    pub fn in_words<'trip>(&self, contents: impl Iterator<Item = &'trip Content>) -> Vec<Finding> {
        contents
            .flat_map(Content::parts)
            .filter_map(Part::words)
            .filter_map(|words| self.entries.found_in(words.as_ref()))
            .map(|name| Finding::whole(Problem::NameInWords { name }))
            .collect()
    }

    #[must_use]
    pub fn unresolved(&self, contents: &[&Content]) -> Vec<Finding> {
        contents
            .iter()
            .flat_map(|content| content.parts())
            .filter_map(|part| self.dangling(part))
            .map(Finding::whole)
            .collect()
    }
}

pub mod disagreement;
mod excerpt;
mod fact;
mod place_fact;
mod ruby_text;
mod source;

use crate::content::{Content, ContentList};
use crate::words::Slug;
use serde::Deserialize;

use crate::english_name::EnglishName;
use crate::naming::Naming;
use disagreement::Disagreement;
pub use excerpt::{Excerpt, ExcerptCredit, ExcerptText};
pub use fact::{Fact, NoteKind};
pub use place_fact::PlaceFact;
pub use ruby_text::RubyText;
pub use source::Source;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Place {
    slug: Slug,
    name: RubyText,
    english: EnglishName,
    #[as_ref]
    tagline: Content,
    meaning: Content,
    origin: Fact,
    geography: Fact,
    excerpts: Vec<Excerpt>,
    disagreements: Vec<Disagreement>,
}

impl Place {
    #[must_use]
    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    #[must_use]
    pub fn gloss(&self) -> &Content {
        &self.meaning
    }

    #[must_use]
    pub fn text(&self, part: Naming) -> &RubyText {
        match part {
            Naming::Name => &self.name,
        }
    }

    #[must_use]
    pub fn english(&self) -> &EnglishName {
        &self.english
    }

    #[must_use]
    pub fn fact(&self, part: PlaceFact) -> &Fact {
        match part {
            PlaceFact::Origin => &self.origin,
            PlaceFact::Geography => &self.geography,
        }
    }

    #[must_use]
    pub fn excerpts(&self) -> &[Excerpt] {
        &self.excerpts
    }

    #[must_use]
    pub fn disagreements(&self) -> &[Disagreement] {
        &self.disagreements
    }
}

impl<'place> From<&'place Place> for ContentList<'place> {
    fn from(place: &'place Place) -> ContentList<'place> {
        let excerpts = place.excerpts.iter().flat_map(|excerpt| {
            std::iter::once(excerpt.text(ExcerptText::Translation)).chain(excerpt.provenance())
        });
        let disagreements = place.disagreements.iter().flat_map(|disagreement| {
            [
                AsRef::<Content>::as_ref(disagreement),
                disagreement.detail(),
            ]
        });
        [&place.tagline, &place.meaning]
            .into_iter()
            .chain(
                [&place.origin, &place.geography]
                    .into_iter()
                    .flat_map(ContentList::from),
            )
            .chain(excerpts)
            .chain(disagreements)
            .collect()
    }
}

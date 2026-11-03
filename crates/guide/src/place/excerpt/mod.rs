mod excerpt_credit;
mod excerpt_text;
mod translator;
mod year;

use crate::content::Content;
use crate::words::Prose;
use serde::Deserialize;

use crate::evidence::Source;
pub use excerpt_credit::ExcerptCredit;
pub use excerpt_text::ExcerptText;
use translator::Translator;
use year::Year;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Excerpt {
    year: Year,
    author: Prose,
    work: Prose,
    translation: Content,
    original: Content,
    translator: Translator,
    source: Source,
    provenance: Option<Content>,
}

impl Excerpt {
    #[must_use]
    pub fn credit(&self, credit: ExcerptCredit) -> &Prose {
        match credit {
            ExcerptCredit::Author => &self.author,
            ExcerptCredit::Work => &self.work,
        }
    }

    #[must_use]
    pub fn year(&self) -> Year {
        self.year
    }

    #[must_use]
    pub fn text(&self, part: ExcerptText) -> &Content {
        match part {
            ExcerptText::Translation => &self.translation,
            ExcerptText::Original => &self.original,
        }
    }

    #[must_use]
    pub fn translator(&self) -> Translator {
        self.translator
    }

    #[must_use]
    pub fn source(&self) -> &Source {
        &self.source
    }

    #[must_use]
    pub fn provenance(&self) -> Option<&Content> {
        self.provenance.as_ref()
    }
}

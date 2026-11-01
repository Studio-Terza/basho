use guide::Language;
use guide::guidance::Guidance;

#[derive(Clone, Copy)]
pub struct Narration<'view> {
    guidance: &'view Guidance<'view>,
    language: Language,
}

impl<'view> Narration<'view> {
    #[must_use]
    pub fn new(guidance: &'view Guidance<'view>, language: Language) -> Narration<'view> {
        Narration { guidance, language }
    }

    #[must_use]
    pub fn guidance(self) -> &'view Guidance<'view> {
        self.guidance
    }

    #[must_use]
    pub fn language(self) -> Language {
        self.language
    }
}

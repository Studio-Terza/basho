use crate::guidance::Pointer;
use crate::place::RubyText;
use crate::words::Prose;

#[derive(Clone, Copy, Debug)]
pub struct CatchSight<'catch> {
    name: &'catch RubyText,
    reference: Option<&'catch Prose>,
    note: Option<&'catch RubyText>,
    pointer: Pointer,
}

impl<'catch> CatchSight<'catch> {
    #[must_use]
    pub fn new(
        name: &'catch RubyText,
        reference: Option<&'catch Prose>,
        note: Option<&'catch RubyText>,
        pointer: Pointer,
    ) -> CatchSight<'catch> {
        CatchSight {
            name,
            reference,
            note,
            pointer,
        }
    }

    #[must_use]
    pub fn name(self) -> &'catch RubyText {
        self.name
    }

    #[must_use]
    pub fn reference(self) -> Option<&'catch Prose> {
        self.reference
    }

    #[must_use]
    pub fn note(self) -> Option<&'catch RubyText> {
        self.note
    }

    #[must_use]
    pub fn pointer(self) -> Pointer {
        self.pointer
    }
}

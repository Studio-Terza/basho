mod depth;

use depth::Depth;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Quoting {
    depth: Depth,
    quote: Option<char>,
}

impl Quoting {
    #[must_use]
    pub fn after(self, character: char, open: char, close: char) -> Quoting {
        let quote = match (self.quote, character) {
            (Some(opened), closing) if opened == closing => None,
            (None, '"' | '\'') => Some(character),
            (kept, _) => kept,
        };
        Quoting {
            depth: self.quote.map_or_else(
                || self.depth.stepped(character, open, close),
                |_| self.depth,
            ),
            quote,
        }
    }

    #[must_use]
    pub fn reaches(self, before: Quoting, character: char, wanted: char) -> bool {
        before.quote.is_none()
            && character == wanted
            && self.depth == Quoting::default().after(wanted, '{', '}').depth
    }

    #[must_use]
    pub fn balanced(self) -> bool {
        self.depth == Depth::default()
    }
}

mod depth;

use depth::Depth;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Quoting {
    depth: Depth,
    quote: Option<char>,
}

impl Quoting {
    const QUOTES: [char; 2] = ['"', '\''];

    #[must_use]
    pub fn after(self, character: char, open: char, close: char) -> Quoting {
        let quote = self.quote.map_or_else(
            || Self::QUOTES.contains(&character).then_some(character),
            |opened| Some(opened).filter(|opened| *opened != character),
        );
        Quoting {
            depth: self.quote.map_or_else(
                || self.depth.stepped(character, open, close),
                |_| self.depth,
            ),
            quote,
        }
    }

    #[must_use]
    pub fn reaching(self, before: Quoting, character: char, wanted: char) -> Option<Quoting> {
        before
            .quote
            .map_or(Some(self), |_| None)
            .filter(|_| character == wanted)
            .filter(|after| after.depth == Quoting::default().after(wanted, '{', '}').depth)
    }

    #[must_use]
    pub fn balanced(self) -> Option<Quoting> {
        Some(self).filter(|state| state.depth == Depth::default())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Depth(i32);

impl Depth {
    pub fn stepped(self, character: char, open: char, close: char) -> Depth {
        Depth(self.0 + i32::from(character == open) - i32::from(character == close))
    }
}

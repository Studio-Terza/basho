use crate::open::Open;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Closed;

impl Closed {
    pub fn open(self) -> Open {
        let Closed = self;
        Open
    }
}

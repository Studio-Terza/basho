use crate::closed::Closed;
use crate::open::Open;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Closing;

impl Closing {
    pub fn settle(self) -> Closed {
        let Closing = self;
        Closed
    }

    pub fn reopen(self) -> Open {
        let Closing = self;
        Open
    }
}

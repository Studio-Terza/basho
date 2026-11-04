use crate::closing::Closing;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Open;

impl Open {
    pub fn close(self) -> Closing {
        let Open = self;
        Closing
    }
}

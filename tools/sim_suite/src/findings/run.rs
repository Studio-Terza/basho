#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    Troubled,
    Failing,
    Crashed,
    Proved,
}

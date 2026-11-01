#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stance {
    Preview,
    Waiting,
    Moving,
    Missed,
    Done,
}

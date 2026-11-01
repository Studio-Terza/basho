#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchLine {
    Spot,
    Trigger,
    Main,
    Backup,
}

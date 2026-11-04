use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Haste {
    #[default]
    DoNotStop,
    JogFromTrain,
}

impl fmt::Display for Haste {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Haste::DoNotStop => formatter.write_str("do not stop"),
            Haste::JogFromTrain => formatter.write_str("jog from the train"),
        }
    }
}

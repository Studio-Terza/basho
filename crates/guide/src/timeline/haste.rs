use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Haste {
    #[default]
    DoNotStop,
    JogFromTrain,
}

impl fmt::Display for Haste {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Haste::DoNotStop => "do not stop",
            Haste::JogFromTrain => "jog from the train",
        })
    }
}

use crate::words::Label;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainKind {
    Local,
    Rapid,
    LimitedExpress,
    Shinkansen,
    Subway,
}

impl From<&TrainKind> for Label {
    fn from(kind: &TrainKind) -> Label {
        match kind {
            TrainKind::Local => Label::new("Local"),
            TrainKind::Rapid => Label::new("Rapid"),
            TrainKind::LimitedExpress => Label::new("Limited express"),
            TrainKind::Shinkansen => Label::new("Shinkansen"),
            TrainKind::Subway => Label::new("Subway"),
        }
    }
}

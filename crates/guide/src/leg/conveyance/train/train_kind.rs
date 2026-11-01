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
        Label::new(match kind {
            TrainKind::Local => "Local",
            TrainKind::Rapid => "Rapid",
            TrainKind::LimitedExpress => "Limited express",
            TrainKind::Shinkansen => "Shinkansen",
            TrainKind::Subway => "Subway",
        })
    }
}

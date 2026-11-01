use crate::task::gate::Detail;
use crate::task::gate::posting::Posting;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusResult {
    Posted,
    WithheldDirtyTree,
    WithheldFailure,
    PostFailed(Detail),
}

impl From<Posting> for StatusResult {
    fn from(posting: Posting) -> StatusResult {
        match posting {
            Posting::Post => StatusResult::Posted,
            Posting::WithheldDirtyTree => StatusResult::WithheldDirtyTree,
            Posting::WithheldFailure => StatusResult::WithheldFailure,
        }
    }
}

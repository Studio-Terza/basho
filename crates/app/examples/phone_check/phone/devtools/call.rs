use serde::Serialize;
use tungstenite::Message;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Call<'call, Params: Serialize> {
    #[serde(rename = "id")]
    index: usize,
    method: &'call str,
    params: &'call Params,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<&'call str>,
}

impl<'call, Params: Serialize> Call<'call, Params> {
    pub fn indexed(
        index: usize,
        method: &'call str,
        params: &'call Params,
        session_id: Option<&'call str>,
    ) -> Self {
        Call {
            index,
            method,
            params,
            session_id,
        }
    }

    pub fn message(&self) -> Message {
        Message::text(serde_json::to_string(self).expect("a DevTools call serializes"))
    }
}

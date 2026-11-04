mod credentials;
mod document;
mod endpoint;
mod identifier;
mod resource;

use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::boundary::Tool;
use crate::failure::Failure;
use crate::xcrun_tool::XcrunTool;
use apple_text::Bearer;
use credentials::Credentials;
pub use document::Document;
pub use endpoint::Endpoint;
pub use resource::Resource;

pub struct Connect {
    credentials: Credentials,
    bearer: Bearer,
}

impl Connect {
    const AGENT: &str = "basho-trip-planner";

    pub fn open() -> Result<Connect, Failure> {
        let credentials = Credentials::load()?;
        let bearer = credentials
            .appended_to(Tool::xcrun(XcrunTool::Altool).arg("--generate-jwt"))
            .transcript()?;
        Ok(Connect {
            credentials,
            bearer,
        })
    }

    pub fn fetch<Content: DeserializeOwned>(
        &self,
        endpoint: &Endpoint<'_>,
    ) -> Result<Content, Failure> {
        let body = self.request(endpoint).text()?;
        serde_json::from_str(&body).map_err(Failure::Response)
    }

    pub fn post<Content: DeserializeOwned>(
        &self,
        endpoint: &Endpoint<'_>,
        body: &impl Serialize,
    ) -> Result<Content, Failure> {
        let body = serde_json::to_string(body).map_err(Failure::Response)?;
        let response = self
            .request(endpoint)
            .args(["-X", "POST", "-H", "Content-Type: application/json"])
            .args(["--data-binary", &body])
            .text()?;
        serde_json::from_str(&response).map_err(Failure::Response)
    }

    pub fn delete(&self, endpoint: &Endpoint<'_>) -> Result<(), Failure> {
        self.request(endpoint)
            .args(["-X", "DELETE"])
            .output()
            .map(drop)
    }

    pub fn upload(&self, package: &Path) -> Result<(), Failure> {
        self.credentials
            .appended_to(
                Tool::xcrun(XcrunTool::Altool)
                    .args(["--upload-app", "--type", "ios", "--file"])
                    .arg(package),
            )
            .status()
    }

    fn request(&self, endpoint: &Endpoint<'_>) -> Tool {
        Tool::named("curl")
            .args(["--silent", "--show-error", "--globoff", "--fail-with-body"])
            .args(["--user-agent", Self::AGENT])
            .arg("--header")
            .rendered(&self.bearer)
            .rendered(endpoint)
    }
}

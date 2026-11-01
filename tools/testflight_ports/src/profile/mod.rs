mod profile_content;

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::connect::{Connect, Document, Endpoint, Resource};
use crate::failure::Failure;
use crate::identity::Identity;
use crate::staging::Staging;
use crate::tool::Tool;
use profile_content::ProfileContent;

pub struct Profile {
    path: PathBuf,
    entitlements: PathBuf,
}

impl Profile {
    const NAME: &str = "Basho Walk App Store";
    const PROFILE_FILE: &str = "embedded.mobileprovision";
    const DECODED_FILE: &str = "profile.plist";
    const ENTITLEMENTS_FILE: &str = "entitlements.plist";

    pub fn issue(
        connect: &Connect,
        identity: &Identity,
        staging: &Staging,
    ) -> Result<Profile, Failure> {
        let bundles: Document<Vec<Resource<Value>>> = connect.fetch(&Endpoint::BashoBundle)?;
        let bundle =
            bundles.into_content().into_iter().next().ok_or_else(|| {
                Failure::Signing("com.cinglein.basho is not registered".to_owned())
            })?;
        Self::retire(connect)?;
        let body = json!({"data": {
            "type": "profiles",
            "attributes": {"name": Self::NAME, "profileType": "IOS_APP_STORE"},
            "relationships": {
                "bundleId": {"data": {"type": "bundleIds", "id": bundle.identifier().as_ref()}},
                "certificates": {"data": [{"type": "certificates", "id": identity.certificate()}]},
            },
        }});
        let created: Document<Resource<ProfileContent>> =
            connect.post(&Endpoint::Profiles, &body)?;
        let path = staging.write(
            Self::PROFILE_FILE,
            created.into_content().attributes().decoded(),
        )?;
        let decoded = staging.write(
            Self::DECODED_FILE,
            &Tool::named("security")
                .args(["cms", "-D", "-i"])
                .arg(&path)
                .output()?,
        )?;
        let entitlements = staging.path(Self::ENTITLEMENTS_FILE);
        Tool::named("plutil")
            .args(["-extract", "Entitlements", "xml1", "-o"])
            .arg(&entitlements)
            .arg(&decoded)
            .output()?;
        Ok(Profile { path, entitlements })
    }

    pub fn embed_into(&self, app: &Path) -> Result<(), Failure> {
        let embedded = app.join(Self::PROFILE_FILE);
        fs::copy(&self.path, &embedded)
            .map(drop)
            .map_err(Staging::io_failure(&embedded))
    }

    pub fn entitlements(&self) -> &Path {
        &self.entitlements
    }

    fn retire(connect: &Connect) -> Result<(), Failure> {
        let stale: Document<Vec<Resource<Value>>> =
            connect.fetch(&Endpoint::ProfilesNamed(Self::NAME))?;
        stale.into_content().iter().try_for_each(|profile| {
            connect.delete(&Endpoint::Profile(profile.identifier().as_ref()))
        })
    }
}

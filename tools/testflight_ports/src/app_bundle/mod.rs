mod boundary;

use std::fs;
use std::path::{Path, PathBuf};

use crate::boundary::Tool;
use crate::failure::Failure;
use crate::identity::Identity;
use crate::profile::Profile;
use crate::staging::Staging;
use boundary::{BuildNumber, IconCatalog, InfoPlist};

pub struct AppBundle {
    path: PathBuf,
    build: BuildNumber,
}

impl AppBundle {
    const CRATE: &str = "crates/app";
    const DX_OUTPUT: &str = "dx";
    const APP: &str = "Basho.app";
    const PAYLOAD: &str = "Payload";
    const PACKAGE: &str = "basho.ipa";
    const DEPLOYMENT_TARGET: &str = "17.0";

    pub fn assemble(root: &Path, staging: &Staging) -> Result<AppBundle, Failure> {
        let dx_output = staging.path(Self::DX_OUTPUT);
        Tool::named("dx")
            .args([
                "bundle",
                "--ios",
                "--release",
                "--target",
                "aarch64-apple-ios",
            ])
            .args(["--package-types", "ios", "--out-dir"])
            .arg(&dx_output)
            .env("IPHONEOS_DEPLOYMENT_TARGET", Self::DEPLOYMENT_TARGET)
            .within(&root.join(Self::CRATE))
            .status()?;
        let payload = staging.path(Self::PAYLOAD);
        let path = payload.join(Self::APP);
        let source = dx_output.join(Self::APP);
        fs::create_dir_all(&payload)
            .and_then(|()| fs::rename(&source, &path))
            .map_err(Staging::io_failure(&source))?;
        let icons = IconCatalog::compile(root, &path, staging, Self::DEPLOYMENT_TARGET)?;
        let build =
            InfoPlist::inside(&path).revise(icons.partial_plist(), Self::DEPLOYMENT_TARGET)?;
        Ok(AppBundle { path, build })
    }

    #[must_use]
    pub fn build_number(&self) -> &BuildNumber {
        &self.build
    }

    pub fn sign(&self, identity: &Identity, profile: &Profile) -> Result<(), Failure> {
        profile.embed_into(&self.path)?;
        Tool::named("codesign")
            .args(["--force", "--sign"])
            .arg(identity.hash())
            .arg("--entitlements")
            .arg(profile.entitlements())
            .arg(&self.path)
            .output()?;
        Tool::named("codesign")
            .args(["--verify", "--strict", "--deep"])
            .arg(&self.path)
            .output()
            .map(drop)
    }

    pub fn archive(&self, staging: &Staging) -> Result<PathBuf, Failure> {
        let package = staging.path(Self::PACKAGE);
        Tool::named("ditto")
            .args(["-c", "-k", "--sequesterRsrc", "--keepParent", Self::PAYLOAD])
            .arg(&package)
            .within(&Self::parent_of(&self.path))
            .output()?;
        Ok(package)
    }

    fn parent_of(app: &Path) -> PathBuf {
        app.parent()
            .and_then(Path::parent)
            .map_or_else(PathBuf::new, Path::to_path_buf)
    }
}

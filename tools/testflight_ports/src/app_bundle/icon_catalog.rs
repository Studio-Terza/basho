use std::path::{Path, PathBuf};

use crate::failure::Failure;
use crate::staging::Staging;
use crate::tool::Tool;
use crate::xcrun_tool::XcrunTool;

pub struct IconCatalog {
    partial_plist: PathBuf,
}

impl IconCatalog {
    const CATALOG: &str = "crates/app/ios/Assets.xcassets";
    const PARTIAL_PLIST: &str = "icon.plist";

    pub fn compile(
        root: &Path,
        app: &Path,
        staging: &Staging,
        deployment_target: &str,
    ) -> Result<IconCatalog, Failure> {
        let partial_plist = staging.path(Self::PARTIAL_PLIST);
        Tool::xcrun(XcrunTool::Actool)
            .arg(root.join(Self::CATALOG))
            .arg("--compile")
            .arg(app)
            .args(["--platform", "iphoneos", "--target-device", "iphone"])
            .args(["--minimum-deployment-target", deployment_target])
            .args(["--app-icon", "AppIcon", "--output-partial-info-plist"])
            .arg(&partial_plist)
            .args([
                "--errors",
                "--warnings",
                "--output-format",
                "human-readable-text",
            ])
            .output()?;
        Ok(IconCatalog { partial_plist })
    }

    pub fn partial_plist(&self) -> &Path {
        &self.partial_plist
    }
}

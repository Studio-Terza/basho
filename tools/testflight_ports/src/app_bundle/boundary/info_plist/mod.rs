mod build_number;
mod merge_command;
mod sdk;

use std::fmt;
use std::path::{Path, PathBuf};

use crate::boundary::Tool;
use crate::failure::Failure;
use apple_text::XcodeVersion;
pub use build_number::BuildNumber;
use merge_command::MergeCommand;
use sdk::Sdk;

pub struct InfoPlist {
    path: PathBuf,
}

impl InfoPlist {
    const DISPLAY_NAME: &str = "Basho Walk";
    const FIXED: [[&str; 3]; 8] = [
        ["CFBundlePackageType", "-string", "APPL"],
        ["CFBundleSupportedPlatforms", "-json", "[\"iPhoneOS\"]"],
        ["UIDeviceFamily", "-json", "[1]"],
        ["UILaunchScreen", "-json", "{}"],
        [
            "UISupportedInterfaceOrientations",
            "-json",
            "[\"UIInterfaceOrientationPortrait\"]",
        ],
        ["DTPlatformName", "-string", "iphoneos"],
        [
            "DTCompiler",
            "-string",
            "com.apple.compilers.llvm.clang.1_0",
        ],
        ["CFBundleDisplayName", "-string", Self::DISPLAY_NAME],
    ];
    const REMOVED: [&str; 2] = ["UILaunchStoryboardName", "UISupportsTrueScreenSizeOnMac"];

    #[must_use]
    pub fn inside(app: &Path) -> InfoPlist {
        InfoPlist {
            path: app.join("Info.plist"),
        }
    }

    pub fn revise(
        &self,
        icon_plist: &Path,
        deployment_target: &str,
    ) -> Result<BuildNumber, Failure> {
        Tool::named("/usr/libexec/PlistBuddy")
            .arg("-c")
            .rendered(&MergeCommand::merging(icon_plist))
            .arg(&self.path)
            .output()?;
        Self::REMOVED
            .iter()
            .try_for_each(|key| self.edit(&["-remove", key]))?;
        Self::FIXED
            .iter()
            .try_for_each(|[key, kind, setting]| self.edit(&["-replace", key, kind, setting]))?;
        let build = BuildNumber::now();
        let sdk = Sdk::query()?;
        let xcode: XcodeVersion = Tool::named("xcodebuild").arg("-version").text()?.parse()?;
        self.replace("CFBundleVersion", &build)?;
        self.replace("MinimumOSVersion", &deployment_target)?;
        sdk.record_in(self)?;
        self.replace("DTXcode", xcode.number())?;
        self.replace("DTXcodeBuild", xcode.build_label())?;
        Ok(build)
    }

    pub fn replace(&self, key: &str, setting: &impl fmt::Display) -> Result<(), Failure> {
        Tool::named("plutil")
            .args(["-replace", key, "-string"])
            .rendered(setting)
            .arg(&self.path)
            .output()
            .map(drop)
    }

    fn edit(&self, arguments: &[&str]) -> Result<(), Failure> {
        Tool::named("plutil")
            .args(arguments)
            .arg(&self.path)
            .output()
            .map(drop)
    }
}

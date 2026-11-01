mod app_bundle;
mod bits;
mod connect;
mod failure;
mod identity;
mod profile;
mod staging;
mod tool;
mod xcrun_tool;

use std::path::Path;

use app_bundle::AppBundle;
use connect::Connect;
use identity::Identity;
use profile::Profile;
use staging::Staging;

pub use failure::Failure;

pub struct Testflight;

impl Testflight {
    pub fn run(root: &Path) -> Result<(), Failure> {
        let staging = Staging::prepare(root)?;
        let connect = Connect::open()?;
        let identity = Identity::ensure(&connect, &staging)?;
        let profile = Profile::issue(&connect, &identity, &staging)?;
        let app = AppBundle::assemble(root, &staging)?;
        app.sign(&identity, &profile)?;
        let package = app.archive(&staging)?;
        println!(
            "testflight: uploading build {} ({})",
            app.build_number(),
            package.display()
        );
        connect.upload(&package)?;
        println!(
            "testflight: build {} uploaded; App Store Connect takes a few minutes to process it before TestFlight offers it",
            app.build_number()
        );
        Ok(())
    }
}

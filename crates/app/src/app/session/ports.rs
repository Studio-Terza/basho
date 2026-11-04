use std::ffi::OsString;

pub struct LaunchArgs;

impl LaunchArgs {
    #[must_use]
    pub fn trip_path() -> Option<OsString> {
        std::env::args_os().nth(1)
    }
}

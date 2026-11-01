use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use crate::task::failure::Failure;
use crate::task::gate::Commit;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use guide::words::Spelling;

pub struct Status;

impl Status {
    const TOOL: &str = "gh";
    const STATUSES: &str = "repos/{owner}/{repo}/statuses/";
    const FIELDS: [&str; 6] = [
        "-f",
        "state=success",
        "-f",
        "context=local-gate",
        "-f",
        "description=cargo xtask gate passed on this commit",
    ];

    pub fn post_success(workspace: &Workspace, commit: &Commit) -> Result<(), Failure> {
        let mut endpoint = OsString::from(Self::STATUSES);
        endpoint.push(Path::as_os_str(commit.as_ref()));
        let output = Command::new(Self::TOOL)
            .args(["api", "--method", "POST"])
            .arg(endpoint)
            .args(Self::FIELDS)
            .current_dir(workspace.root())
            .output()
            .map_err(|error| Failure::Io {
                site: Site::Workspace,
                error,
            })?;
        output
            .status
            .success()
            .then_some(())
            .ok_or_else(|| Failure::Tool {
                tool: Spelling::new(Self::TOOL.to_owned()),
                detail: Spelling::new(String::from_utf8_lossy(&output.stderr).into_owned()),
            })
    }
}

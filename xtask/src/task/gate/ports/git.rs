use std::process::Command;

use crate::task::failure::Failure;
use crate::task::gate::{Commit, Tree};
use crate::task::workspace::Workspace;
use basho_lints::Site;
use guide::words::Spelling;

pub struct Git;

impl Git {
    const TOOL: &str = "git";
    const HEAD: [&str; 2] = ["rev-parse", "HEAD"];
    const PORCELAIN: [&str; 2] = ["status", "--porcelain"];

    pub fn head(workspace: &Workspace) -> Result<Commit, Failure> {
        Self::printed(workspace, &Self::HEAD)?.parse::<Commit>()
    }

    pub fn tree(workspace: &Workspace) -> Result<Tree, Failure> {
        Ok(Self::printed(workspace, &Self::PORCELAIN)?
            .parse::<Tree>()
            .unwrap_or(Tree::Dirty))
    }

    fn printed(workspace: &Workspace, arguments: &[&str]) -> Result<String, Failure> {
        let output = Command::new(Self::TOOL)
            .args(arguments)
            .current_dir(workspace.root())
            .output()
            .map_err(|error| Failure::Io {
                site: Site::Workspace,
                error,
            })?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
            .ok_or_else(|| Failure::Tool {
                tool: Spelling::new(Self::TOOL.to_owned()),
                detail: Spelling::new(String::from_utf8_lossy(&output.stderr).into_owned()),
            })
    }
}

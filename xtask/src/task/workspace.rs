use std::path::{Path, PathBuf};

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn locate() -> Workspace {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask lives directly under the workspace root")
            .to_path_buf();
        Workspace { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

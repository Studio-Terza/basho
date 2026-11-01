mod crate_name;
mod relative_path;
mod source_text;

use std::fs;
use std::path::Path;

use crate::failure::Failure;
use crate::site::Site;
pub use crate_name::CrateName;
pub use relative_path::RelativePath;
pub use source_text::SourceText;

pub struct SourceFile {
    path: RelativePath,
    text: SourceText,
    syntax: syn::File,
}

impl SourceFile {
    const TEST_TARGETS: [&str; 2] = ["tests", "examples"];

    pub fn read(root: &Path, file: &Path) -> Result<Self, Failure> {
        let path = RelativePath::under(root, file);
        let text = fs::read_to_string(file).map_err(|error| Failure::Io {
            site: Site::File(path.clone()),
            error,
        })?;
        Self::parse(path, SourceText::new(text))
    }

    pub fn parse(path: RelativePath, text: SourceText) -> Result<Self, Failure> {
        let syntax = syn::parse_file(&text).map_err(|error| Failure::Parse {
            site: Site::File(path.clone()),
            error,
        })?;
        Ok(Self { path, text, syntax })
    }

    #[must_use]
    pub fn path(&self) -> &RelativePath {
        &self.path
    }

    #[must_use]
    pub fn crate_name(&self) -> Option<CrateName<'_>> {
        match self.path.strip_prefix("crates/") {
            Some(inside) => inside.split('/').next(),
            None => self.path.strip_prefix("xtask/").map(|_| "xtask"),
        }
        .map(CrateName::new)
    }

    #[must_use]
    pub fn is_test_target(&self) -> bool {
        Path::new(&*self.path)
            .iter()
            .any(|part| Self::TEST_TARGETS.iter().any(|target| part == *target))
    }

    #[must_use]
    pub fn text(&self) -> &SourceText {
        &self.text
    }

    #[must_use]
    pub fn syntax(&self) -> &syn::File {
        &self.syntax
    }

    #[must_use]
    pub fn is_test_item(attributes: &[syn::Attribute]) -> bool {
        attributes.iter().any(|attribute| {
            attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Ident>()
                    .is_ok_and(|ident| ident == "test")
        })
    }
}

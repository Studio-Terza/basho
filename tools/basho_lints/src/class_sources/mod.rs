mod class_scan;
mod enum_names;

use syn::visit::Visit;

use crate::SourceRule;
use crate::report::Report;
use crate::source_file::{CrateName, SourceFile};
use class_scan::ClassScan;
use enum_names::EnumNames;

pub struct ClassSources;

impl ClassSources {
    const UI_CRATES: [CrateName<'static>; 4] = [
        CrateName::new("app"),
        CrateName::new("guide"),
        CrateName::new("layout"),
        CrateName::new("menu"),
    ];
}

impl SourceRule for ClassSources {
    fn report(files: &[SourceFile]) -> Report {
        let enums = EnumNames::in_files(files);
        Report::new(
            "markup that CSS can select on (class, style, role, lang, data-*) never carries instance data: a class is a literal or .class() on a closed enum with no if or match, style is static or a typed pose, and role, lang and data-* are static strings",
            files
                .iter()
                .filter(|file| {
                    file.crate_name()
                        .is_some_and(|name| Self::UI_CRATES.contains(&name))
                })
                .flat_map(|file| {
                    let mut scan = ClassScan::new(file.path(), &enums);
                    scan.visit_file(file.syntax());
                    scan.found()
                })
                .collect(),
        )
    }
}

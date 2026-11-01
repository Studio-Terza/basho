pub mod class_sources;
mod failure;
mod line_number;
pub mod ownership;
mod report;
mod site;
pub mod source_file;
mod source_tree;
pub mod ui_numbers;
mod violation;

use source_file::SourceFile;

pub use failure::Failure;
pub use line_number::LineNumber;
pub use report::Report;
pub use site::Site;
pub use source_tree::SourceTree;
pub use violation::Violation;

pub trait SourceRule {
    fn report(files: &[SourceFile]) -> Report;
}

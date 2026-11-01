use proc_macro2::Ident;
use syn::ItemEnum;
use syn::visit::Visit;

use crate::source_file::SourceFile;

#[derive(Default)]
pub struct EnumNames {
    names: Vec<Ident>,
}

impl EnumNames {
    pub fn in_files(files: &[SourceFile]) -> Vec<Ident> {
        files
            .iter()
            .fold(EnumNames::default(), |mut found, file| {
                found.visit_file(file.syntax());
                found
            })
            .names
    }
}

impl<'ast> Visit<'ast> for EnumNames {
    fn visit_item_enum(&mut self, item: &'ast ItemEnum) {
        self.names.push(item.ident.clone());
        syn::visit::visit_item_enum(self, item);
    }
}

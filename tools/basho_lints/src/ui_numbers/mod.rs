use proc_macro2::{TokenStream, TokenTree};
use syn::visit::Visit;
use syn::{
    ExprRepeat, Fields, ImplItemFn, ItemFn, ItemMod, ItemStruct, Lit, LitFloat, LitInt, Macro,
    Type, TypeArray,
};

use crate::SourceRule;
use crate::line_number::LineNumber;
use crate::report::Report;
use crate::source_file::{CrateName, RelativePath, SourceFile};
use crate::violation::Violation;

pub struct UiNumbers;

impl UiNumbers {
    const UI_CRATES: [CrateName<'static>; 3] = [
        CrateName::new("app"),
        CrateName::new("layout"),
        CrateName::new("menu"),
    ];
    const OWNERS: [&str; 8] = [
        "crates/app/src/app/session/measure/",
        "crates/app/src/app/session/hand/",
        "crates/layout/src/measure/",
        "crates/layout/src/scale/",
        "crates/layout/src/mark_size.rs",
        "crates/layout/src/motion.rs",
        "crates/layout/src/panel.rs",
        "crates/layout/src/touch.rs",
    ];
}

impl SourceRule for UiNumbers {
    fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "a UI number is a measure: name it as a qualitative token in measure, scale or hand, or write it as a formula of those",
            files
                .iter()
                .filter(|file| {
                    file.crate_name()
                        .is_some_and(|name| Self::UI_CRATES.contains(&name))
                })
                .filter(|file| !file.is_test_target())
                .filter(|file| {
                    !Self::OWNERS
                        .iter()
                        .any(|owner| file.path().starts_with(owner))
                })
                .flat_map(|file| {
                    let mut scan = Literals::new(file.path());
                    scan.visit_file(file.syntax());
                    match scan.owner {
                        Owner::Plain => scan.found,
                        Owner::Newtype => Vec::new(),
                    }
                })
                .collect(),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Owner {
    Plain,
    Newtype,
}

impl Owner {
    const PRIMITIVES: [&str; 12] = [
        "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize", "f32", "f64",
    ];

    fn declared_by(fields: &Fields) -> Owner {
        let primitive = |field: &syn::Field| matches!(&field.ty, Type::Path(path) if Self::PRIMITIVES.iter().any(|name| path.path.is_ident(name)));
        match fields {
            Fields::Unnamed(fields)
                if fields.unnamed.len() == 1 && fields.unnamed.iter().all(primitive) =>
            {
                Owner::Newtype
            }
            _ => Owner::Plain,
        }
    }
}

struct Literals<'file> {
    path: &'file RelativePath,
    found: Vec<Violation>,
    owner: Owner,
}

impl Literals<'_> {
    const STRUCTURAL: [&'static str; 7] = ["0", "1", "2", "0.0", "1.0", "2.0", "0.5"];

    fn new(path: &RelativePath) -> Literals<'_> {
        Literals {
            path,
            found: Vec::new(),
            owner: Owner::Plain,
        }
    }

    fn record(&mut self, line: LineNumber, digits: &str) {
        self.found.extend(
            (!Self::STRUCTURAL.contains(&digits))
                .then(|| Violation::new(line.site(self.path), digits)),
        );
    }

    fn literal(&mut self, literal: &proc_macro2::Literal) {
        let line = LineNumber::from(literal.span().start());
        match Lit::new(literal.clone()) {
            Lit::Int(number) => self.record(line, number.base10_digits()),
            Lit::Float(number) => self.record(line, number.base10_digits()),
            _ => (),
        }
    }

    fn scan_tokens(&mut self, tokens: TokenStream) {
        tokens.into_iter().for_each(|token| match token {
            TokenTree::Group(group) => self.scan_tokens(group.stream()),
            TokenTree::Literal(literal) => self.literal(&literal),
            TokenTree::Ident(_) | TokenTree::Punct(_) => (),
        });
    }
}

impl<'ast> Visit<'ast> for Literals<'_> {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        Some(module)
            .filter(|module| !SourceFile::is_test_item(&module.attrs))
            .into_iter()
            .for_each(|module| syn::visit::visit_item_mod(self, module));
    }

    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        Some(function)
            .filter(|function| !SourceFile::is_test_item(&function.attrs))
            .into_iter()
            .for_each(|function| syn::visit::visit_item_fn(self, function));
    }

    fn visit_impl_item_fn(&mut self, function: &'ast ImplItemFn) {
        Some(function)
            .filter(|function| !SourceFile::is_test_item(&function.attrs))
            .into_iter()
            .for_each(|function| {
                syn::visit::visit_impl_item_fn(self, function);
            });
    }

    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        self.owner = match (self.owner, Owner::declared_by(&item.fields)) {
            (Owner::Plain, Owner::Plain) => Owner::Plain,
            (Owner::Newtype, _) | (_, Owner::Newtype) => Owner::Newtype,
        };
        syn::visit::visit_item_struct(self, item);
    }

    fn visit_type_array(&mut self, array: &'ast TypeArray) {
        self.visit_type(&array.elem);
    }

    fn visit_expr_repeat(&mut self, repeat: &'ast ExprRepeat) {
        self.visit_expr(&repeat.expr);
    }

    fn visit_macro(&mut self, mac: &'ast Macro) {
        self.scan_tokens(mac.tokens.clone());
    }

    fn visit_lit_int(&mut self, literal: &'ast LitInt) {
        self.record(
            LineNumber::from(literal.span().start()),
            literal.base10_digits(),
        );
    }

    fn visit_lit_float(&mut self, literal: &'ast LitFloat) {
        self.record(
            LineNumber::from(literal.span().start()),
            literal.base10_digits(),
        );
    }
}

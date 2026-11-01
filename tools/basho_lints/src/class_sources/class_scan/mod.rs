mod class_fn;
mod class_problem;
mod hook;

use proc_macro2::{Group, Ident, Spacing, TokenStream, TokenTree};
use syn::visit::Visit;
use syn::{ImplItem, ImplItemFn, ItemFn, ItemImpl, Local, Macro, Pat, Type};

use crate::line_number::LineNumber;
use crate::source_file::RelativePath;
use crate::violation::Violation;
use class_fn::ClassFn;
use hook::{Hook, Placeholders};

pub struct ClassScan<'file> {
    path: &'file RelativePath,
    enums: &'file [Ident],
    poses: Vec<Ident>,
    geometry: &'static [&'static str],
    found: Vec<Violation>,
}

impl<'file> ClassScan<'file> {
    const MACRO: &'static str = "rsx";
    const CLOSED_METHODS: [&'static str; 2] = ["class", "flag"];
    const POSE_TYPES: [&'static str; 2] = ["PoseStyle", "PageShift"];
    const NAME_AND_COLON_LEN: usize = 2;
    const MAP_DRAWING: [&'static str; 3] = [
        "crates/map_plan/src/map_mark/",
        "crates/app/src/app/screen/map_view/day_map/map_key_view.rs",
        "crates/app/src/app/screen/map_view/arrow_drawing.rs",
    ];
    const GEOMETRY: [&'static str; 22] = [
        "view_box",
        "preserve_aspect_ratio",
        "width",
        "height",
        "x",
        "y",
        "x1",
        "y1",
        "x2",
        "y2",
        "cx",
        "cy",
        "r",
        "rx",
        "ry",
        "d",
        "points",
        "transform",
        "font_size",
        "letter_spacing",
        "text_anchor",
        "stroke_width",
    ];

    pub fn new(path: &'file RelativePath, enums: &'file [Ident]) -> ClassScan<'file> {
        ClassScan {
            path,
            enums,
            poses: Vec::new(),
            geometry: if Self::MAP_DRAWING
                .iter()
                .any(|drawing| path.starts_with(drawing))
            {
                &Self::GEOMETRY
            } else {
                &[]
            },
            found: Vec::new(),
        }
    }

    pub fn found(self) -> Vec<Violation> {
        self.found
    }

    pub fn scan(&mut self, tokens: TokenStream) {
        let trees: Vec<TokenTree> = tokens.into_iter().collect();
        self.descend(&trees);
    }

    pub fn element(&mut self, body: &Group) {
        let trees: Vec<TokenTree> = body.stream().into_iter().collect();
        let refused: Vec<Violation> = trees
            .iter()
            .enumerate()
            .filter_map(|(index, tree)| {
                let hook = self.hook_at(trees[..index].last(), &trees[index..])?;
                let value: Vec<TokenTree> = trees[index + Self::NAME_AND_COLON_LEN..]
                    .iter()
                    .take_while(
                        |tree| !matches!(tree, TokenTree::Punct(punct) if punct.as_char() == ','),
                    )
                    .cloned()
                    .collect();
                let line = LineNumber::from(tree.span().start());
                hook.refusal(&value, &self.poses)
                    .map(|refusal| Violation::new(line.site(self.path), refusal))
            })
            .collect();
        self.found.extend(refused);
        self.descend(&trees);
    }

    pub fn descend(&mut self, trees: &[TokenTree]) {
        let groups: Vec<(Option<&TokenTree>, &Group)> = trees
            .iter()
            .enumerate()
            .filter_map(|(index, tree)| match tree {
                TokenTree::Group(group) => Some((trees[..index].last(), group)),
                TokenTree::Ident(_) | TokenTree::Punct(_) | TokenTree::Literal(_) => None,
            })
            .collect();
        let (elements, others): (Vec<_>, Vec<_>) = groups
            .into_iter()
            .partition(|(before, _)| Placeholders::opens_element(*before));
        elements
            .into_iter()
            .map(|(_, group)| group)
            .for_each(|group| self.element(group));
        others
            .into_iter()
            .map(|(_, group)| group.stream())
            .for_each(|stream| self.scan(stream));
    }

    fn hook_at(&self, before: Option<&TokenTree>, rest: &[TokenTree]) -> Option<Hook> {
        rest.first()
            .filter(|_| {
                before.is_none_or(
                    |before| matches!(before, TokenTree::Punct(punct) if punct.as_char() == ','),
                ) && matches!(rest.get(1), Some(TokenTree::Punct(colon)) if colon.as_char() == ':' && colon.spacing() == Spacing::Alone)
            })
            .and_then(|name| Hook::named_by(name, |name| self.geometry.contains(&&**name)))
    }

    fn pose_name(local: &Local) -> Option<Ident> {
        let typed = match &local.pat {
            Pat::Type(typed) => Some(typed),
            _ => None,
        }?;
        match (typed.pat.as_ref(), typed.ty.as_ref()) {
            (Pat::Ident(name), Type::Path(path))
                if path.path.segments.last().is_some_and(|segment| {
                    Self::POSE_TYPES.iter().any(|pose| segment.ident == pose)
                }) =>
            {
                Some(name.ident.clone())
            }
            _ => None,
        }
    }

    fn scoped(&mut self, visit: impl FnOnce(&mut Self)) {
        let outer = std::mem::take(&mut self.poses);
        visit(self);
        self.poses = outer;
    }
}

impl<'ast> Visit<'ast> for ClassScan<'_> {
    fn visit_macro(&mut self, called: &'ast Macro) {
        Some(called)
            .filter(|called| {
                called
                    .path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == Self::MACRO)
            })
            .into_iter()
            .for_each(|called| self.scan(called.tokens.clone()));
    }

    fn visit_local(&mut self, local: &'ast Local) {
        self.poses.extend(Self::pose_name(local));
        syn::visit::visit_local(self, local);
    }

    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        self.scoped(|scan| syn::visit::visit_item_fn(scan, function));
    }

    fn visit_impl_item_fn(&mut self, function: &'ast ImplItemFn) {
        self.scoped(|scan| syn::visit::visit_impl_item_fn(scan, function));
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        let refused: Vec<Violation> = item
            .items
            .iter()
            .filter_map(|member| match member {
                ImplItem::Fn(function)
                    if Self::CLOSED_METHODS
                        .iter()
                        .any(|method| function.sig.ident == method) =>
                {
                    Some(function)
                }
                _ => None,
            })
            .filter_map(|function| {
                ClassFn::problem(item, function, self.enums).map(|problem| {
                    Violation::new(
                        LineNumber::from(function.sig.ident.span().start()).site(self.path),
                        problem,
                    )
                })
            })
            .collect();
        self.found.extend(refused);
        self.found.extend(
            ClassFn::conversion_problem(item, self.enums).map(|problem| {
                Violation::new(
                    LineNumber::from(item.impl_token.span.start()).site(self.path),
                    problem,
                )
            }),
        );
        syn::visit::visit_item_impl(self, item);
    }
}

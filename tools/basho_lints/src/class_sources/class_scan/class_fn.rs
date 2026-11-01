use proc_macro2::{Ident, Span};
use syn::{Expr, ExprMatch, ImplItemFn, ItemImpl, Lit, ReturnType, Stmt, Type};

use super::class_problem::ClassProblem;

pub struct ClassFn;

impl ClassFn {
    const RECEIVER: &str = "self";
    const UNNAMED: &str = "unnamed";
    const STATIC: &str = "static";
    const TEXT: &str = "str";
    const FLAG: &str = "flag";
    const NOTHING: &str = "None";
    const SOME: &str = "Some";
    const CONVERSION: &str = "From";
    const CLASSED: &str = "Classed";

    pub fn problem(
        item: &ItemImpl,
        function: &ImplItemFn,
        enums: &[Ident],
    ) -> Option<ClassProblem> {
        let owner = match item.self_ty.as_ref() {
            Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.clone()),
            _ => None,
        }
        .unwrap_or_else(|| Ident::new(Self::UNNAMED, Span::call_site()));
        let choice = match function.block.stmts.as_slice() {
            [Stmt::Expr(Expr::Match(choice), None)] => Some(choice),
            _ => None,
        };
        if function.sig.ident != Self::FLAG
            && !matches!(&function.sig.output, ReturnType::Type(_, returned) if matches!(returned.as_ref(), Type::Reference(reference) if reference.lifetime.as_ref().is_some_and(|lifetime| lifetime.ident == Self::STATIC) && matches!(reference.elem.as_ref(), Type::Path(path) if path.path.is_ident(Self::TEXT))))
        {
            Some(ClassProblem::Returned)
        } else if enums.contains(&owner) && choice.is_some_and(Self::on_self) {
            None
        } else {
            Some(ClassProblem::Unclosed(owner))
        }
    }

    pub fn conversion_problem(item: &ItemImpl, enums: &[Ident]) -> Option<ClassProblem> {
        let named = item
            .trait_
            .as_ref()
            .and_then(|(_, path, _)| path.segments.last())
            .map(|segment| &segment.ident);
        match (named, item.self_ty.as_ref()) {
            (Some(name), Type::Reference(reference))
                if name == Self::CONVERSION
                    && reference
                        .lifetime
                        .as_ref()
                        .is_some_and(|lifetime| lifetime.ident == Self::STATIC) =>
            {
                Some(ClassProblem::HandConverted)
            }
            (Some(name), Type::Path(path)) if name == Self::CLASSED => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.clone())
                .filter(|owner| !enums.contains(owner))
                .map(ClassProblem::OpenClassed),
            _ => None,
        }
    }

    fn literal(expr: &Expr) -> bool {
        match expr {
            Expr::Lit(literal) => matches!(literal.lit, Lit::Str(_) | Lit::Bool(_)),
            Expr::Path(path) => path.path.is_ident(Self::NOTHING),
            Expr::Call(call) => {
                matches!(call.func.as_ref(), Expr::Path(path) if path.path.is_ident(Self::SOME))
                    && call.args.iter().all(Self::literal)
            }
            _ => false,
        }
    }

    fn on_self(choice: &ExprMatch) -> bool {
        let receiver = match choice.expr.as_ref() {
            Expr::Unary(unary) => unary.expr.as_ref(),
            expr => expr,
        };
        matches!(receiver, Expr::Path(path) if path.path.is_ident(Self::RECEIVER))
            && choice.arms.iter().all(|arm| Self::literal(&arm.body))
    }
}

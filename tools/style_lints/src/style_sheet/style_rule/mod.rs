mod declaration;
mod selector;

use basho_lints::LineNumber;
pub use declaration::{CssValue, Declaration};
pub use selector::{CssClass, Selector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Context {
    Plain,
    Starting,
    Nested,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StyleRule {
    selector: Selector,
    declarations: Vec<Declaration>,
    line: LineNumber,
    context: Context,
}

impl StyleRule {
    #[must_use]
    pub fn new(
        selector: Selector,
        declarations: Vec<Declaration>,
        line: LineNumber,
        context: Context,
    ) -> StyleRule {
        StyleRule {
            selector,
            declarations,
            line,
            context,
        }
    }

    #[must_use]
    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    #[must_use]
    pub fn declarations(&self) -> &[Declaration] {
        &self.declarations
    }

    #[must_use]
    pub fn line(&self) -> LineNumber {
        self.line
    }

    #[must_use]
    pub fn context(&self) -> Context {
        self.context
    }
}

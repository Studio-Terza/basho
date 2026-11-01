use std::fmt;

use super::selector_parts::SimplePart;
use crate::style_sheet::Selector;

pub struct Picked<'rule> {
    selector: &'rule Selector,
    part: SimplePart,
}

impl<'rule> Picked<'rule> {
    pub fn new(selector: &'rule Selector, part: SimplePart) -> Picked<'rule> {
        Picked { selector, part }
    }
}

impl fmt::Display for Picked<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.selector, self.part)
    }
}

use newtype::Newtype;
use std::fmt;

#[derive(Newtype)]
pub struct ConditionClause<'text> {
    markup: &'text str,
}

impl fmt::Display for ConditionClause<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trimmed = self.markup.trim_end();
        let clipped = trimmed.strip_suffix('.').unwrap_or(trimmed);
        clipped.chars().next().map_or(Ok(()), |first| {
            write!(
                formatter,
                "{}{}",
                first.to_uppercase(),
                &clipped[first.len_utf8()..]
            )
        })
    }
}

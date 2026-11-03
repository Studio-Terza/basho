mod corner;
mod line;
mod screen_share;
mod span;
mod target;
mod text;
pub use line::Line;
pub use target::Target;
pub use text::Text;

use newtype::Newtype;
use std::fmt;

use crate::measure::Extent;
use crate::scale::Gap;
use crate::scale::Token;
use crate::scale::Viewport;
use corner::Corner;
use screen_share::ScreenShare;
use span::Span;

#[derive(Newtype, Clone, Copy, Debug, PartialEq)]
pub struct RootStyle {
    viewport: Viewport,
}

impl RootStyle {
    const EASE: &str = "cubic-bezier(0.32, 0.72, 0, 1)";

    fn lengths<T: Token>(self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::ALL.iter().try_for_each(|token| {
            write!(
                formatter,
                "--{}: {}px; ",
                token.name(),
                token.length(self.viewport)
            )
        })
    }
}

impl fmt::Display for RootStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, ":root {{ ")?;
        self.lengths::<Gap>(formatter)?;
        self.lengths::<Line>(formatter)?;
        self.lengths::<Corner>(formatter)?;
        self.lengths::<Text>(formatter)?;
        self.lengths::<Extent>(formatter)?;
        self.lengths::<Target>(formatter)?;
        Span::ALL
            .iter()
            .try_for_each(|span| write!(formatter, "--{}: {span}; ", span.name()))?;
        ScreenShare::ALL
            .iter()
            .try_for_each(|share| write!(formatter, "--{}: {share}; ", share.name()))?;
        write!(
            formatter,
            "--ease: {}; --reading: {}; }}",
            Self::EASE,
            self.viewport.reading().factor()
        )
    }
}

use std::fmt;

use guide::leg::conveyance::RouteReason;
use guide::words::Label;
use newtype::Newtype;

use crate::days_html::day_page::ruby_markup::RubyMarkup;
use crate::days_html::day_page::unverified_markup::UnverifiedMarkup;

#[derive(Newtype)]
pub struct ReasonMarkup<'reason> {
    reason: &'reason RouteReason,
}

impl fmt::Display for ReasonMarkup<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<br><b>Why this way:</b> ")?;
        let stated = self
            .reason
            .kind()
            .zip(self.reason.referent())
            .map(|(kind, referent)| {
                format!(
                    "{} · {}{}",
                    Label::from(&kind),
                    RubyMarkup::new(referent.as_ref()),
                    self.reason
                        .evidence()
                        .map_or(String::new(), |evidence| UnverifiedMarkup::new(evidence)
                            .to_string())
                )
            });
        formatter.write_str(stated.as_deref().unwrap_or("No reason recorded"))
    }
}

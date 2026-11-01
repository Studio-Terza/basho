use std::fmt;

use guide::leg::conveyance::{Headsign, LineName, Platform, Train, TrainKind};
use guide::words::Label;
use newtype::Newtype;

use crate::days_html::day_page::ruby_markup::RubyMarkup;
use crate::days_html::day_page::unverified_markup::UnverifiedMarkup;

#[derive(Newtype)]
pub struct BoardMarkup<'train> {
    train: &'train Train,
}

impl fmt::Display for BoardMarkup<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let platform: &Platform = self.train.as_ref();
        let headsign: &Headsign = self.train.as_ref();
        let line: &LineName = self.train.as_ref();
        let kind: &TrainKind = self.train.as_ref();
        write!(
            formatter,
            "<b>{platform}</b> · <b>「{}」</b> · {} · {}{}<br>",
            RubyMarkup::new(headsign.as_ref()),
            RubyMarkup::new(line.as_ref()),
            Label::from(kind),
            UnverifiedMarkup::new(self.train.as_ref())
        )
    }
}

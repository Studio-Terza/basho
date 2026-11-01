use std::iter;
use std::str::FromStr;

use crate::text_failure::{TextFailure, Written};

#[derive(newtype::Display)]
pub struct XcodeNumber(String);

#[derive(newtype::Display)]
pub struct XcodeBuild(String);

pub struct XcodeVersion {
    number: XcodeNumber,
    build: XcodeBuild,
}

impl XcodeVersion {
    const PARTS_LENGTH: usize = 3;
    const NAME_PREFIX: &str = "Xcode ";
    const BUILD_PREFIX: &str = "Build version ";

    #[must_use]
    pub fn number(&self) -> &XcodeNumber {
        &self.number
    }

    #[must_use]
    pub fn build_label(&self) -> &XcodeBuild {
        &self.build
    }
}

impl FromStr for XcodeVersion {
    type Err = TextFailure;

    fn from_str(text: &str) -> Result<XcodeVersion, TextFailure> {
        let unreadable = || TextFailure::UnreadableVersion(Written::new(text.to_owned()));
        let version = text
            .lines()
            .find_map(|line| line.strip_prefix(Self::NAME_PREFIX))
            .ok_or_else(unreadable)?;
        let build = text
            .lines()
            .find_map(|line| line.strip_prefix(Self::BUILD_PREFIX))
            .ok_or_else(unreadable)?
            .trim()
            .to_owned();
        let parts: Vec<&str> = version
            .trim()
            .split('.')
            .chain(iter::repeat("0"))
            .take(Self::PARTS_LENGTH)
            .collect();
        let number = parts
            .split_first()
            .map(|(major, rest)| format!("{major:0>2}{}", rest.concat()))
            .unwrap_or_default();
        Ok(XcodeVersion {
            number: XcodeNumber(number),
            build: XcodeBuild(build),
        })
    }
}

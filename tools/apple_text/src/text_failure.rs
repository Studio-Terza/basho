use thiserror::Error;

#[derive(newtype::Newtype, newtype::Arbitrary, newtype::Display, Debug)]
pub struct Written(String);

#[derive(Debug, Error)]
pub enum TextFailure {
    #[error("App Store Connect: altool printed no token")]
    NoToken,
    #[error("signing: not a SHA-1 fingerprint: {0}")]
    NotFingerprint(Written),
    #[error("signing: unreadable xcodebuild -version: {0}")]
    UnreadableVersion(Written),
}

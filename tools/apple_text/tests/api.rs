use apple_text::{Bearer, Fingerprint, XcodeVersion};
use laws::proptest::prelude::Strategy;

#[test]
fn packs_the_version_like_xcode_does() {
    let current: XcodeVersion = "Xcode 27.0\nBuild version 27A266a\n".parse().unwrap();
    let patched: XcodeVersion = "Xcode 9.4.1\nBuild version 9F2000\n".parse().unwrap();
    assert_eq!(
        [current.number().to_string(), patched.number().to_string()],
        ["2700", "0941"]
    );
}

laws::roundtrip!(
    a_bearer_reads_back_from_its_header,
    "eyJ[A-Za-z0-9._-]{0,40}".prop_map(|token| token.parse::<Bearer>().unwrap()),
    Bearer::to_string,
    |header: &String| header.parse::<Bearer>().ok(),
    Some,
);

laws::roundtrip!(
    a_fingerprint_reads_back_from_its_text,
    "[0-9A-F]{40}".prop_map(|hash| hash.parse::<Fingerprint>().unwrap()),
    Fingerprint::to_string,
    |hash: &String| hash.parse::<Fingerprint>().ok(),
    Some,
);

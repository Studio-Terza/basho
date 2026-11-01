# lint-adoption

Problem: Basho should run the shared lints, as the second test of whether they are any good.

L1 The app's behaviour does not change. Goldens are unchanged.
L2 No allow-lists or lint configuration.
L3 Tests go through public APIs.
D1 Local gate runs the `lint` binary from Studio-Terza/meta-dev at commit 363f17718ef941470309e1cf8805e01ad5b3bbab.
D2 Fix violations, by crate.
D3 Remove meta-xtask lints once the ported lints pass. after: D2
Out: new features, UX changes, trip content, TestFlight.

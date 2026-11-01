# trip-file

Problem: the app hardcodes one trip. It should take a trip file as input.

L1 Every trip fact comes from the file; none comes from code.
L2 A bad trip file fails at load with a clear error, never mid-trip.
L3 The existing trip renders the same as before, proved by a black-box check.
D1 File format is TOML, unless the current data makes another format plainly simpler.
D2 A second, small trip file renders with no code change. after: L1
Out: UX changes, trip editing, new features, TestFlight, changes to the trip's content.

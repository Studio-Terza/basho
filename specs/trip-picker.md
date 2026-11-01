# trip-picker

Problem: a trip is chosen at build time, so the app is not yet a general travel app.

L1 A user can load a trip file on the phone, offline, with no rebuild.
L2 A bad file shows a clear error and leaves the current trip untouched.
L3 Existing behaviour with the built-in trip is unchanged, proved by goldens.
D1 Load a trip from a file at runtime.
D2 A plain picker, reached from the left-edge menu, no popups. after: D1
D3 The chosen trip persists across launches. after: D1
Out: trip editing, sync, sharing, multiple simultaneous trips, TestFlight.

# set-now

Problem: the tracker jumped backward or refused to advance, and there was no manual fix.

L1 A manual set is never overridden backward by the tracker.
L2 The tracker still advances forward by itself.
L3 No popups.
D1 Tracker state as a typed lifecycle: automatic, manually set. Use `states!` from Studio-Terza/meta-dev if it helps.
D2 Done button on the current step. after: D1
D3 I'm here button while browsing any step. after: D1
D4 Going back is only by the user's explicit action. after: D1
Out: time setting, other tracker logic changes, other UX.

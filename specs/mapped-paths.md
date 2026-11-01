# mapped-paths

Problem: routes showed lanes as connected when they were not.

L1 A walking segment is either mapped, on real connected ways, or an explicit no-mapped-path segment. Nothing else can be constructed.
L2 A no-mapped-path segment shows a visible warning, distinct without colour.
L3 Data checks fail any line that uses a non-existent connection.
D1 Segment types and the trip file format.
D2 The data check, run in the gate. after: D1
D3 Rendering and warning. after: D1
D4 Day 7 legs 15 to 17 re-routed on real roads, using the repo's existing map data. after: D2
Out: new map engine, other route changes, tiles.

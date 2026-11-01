# vector-maps

Problem: maps were incomplete, road size was unclear, and the renderer drained the battery.

L1 Maps work fully offline.
L2 Road class reads without colour: width for importance, double lines for major, solid paved, dashed unpaved, dotted trail.
L3 Our own layer draws only route, steps, markers and walker.
L4 No full re-render per position fix.
D1 Measure first: offline tile size for the trip corridor, recorded in the PR, from a real OSM extract.
D2 Bundled tiles and MapLibre in the WebView. after: D1
D3 Greyscale style by road class, and a key tied to what's on screen. after: D2
D4 Guide mode: framed, pinch changes level leg, goal, day. after: D2
D5 Explore mode, with Guide returning. after: D4
D6 Battery: distance filter, pause when still, compass rate limit, progress writes about once a minute. after: D2
Out: online tiles, search, routing, new trip content.

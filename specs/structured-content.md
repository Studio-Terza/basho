# structured-content

Problem: free text in trip data bypasses every typed guarantee.

L1 No free-text field in the trip file. Content is a list of typed parts: time, place, entry link, distance, and so on, with connecting words as the smallest leaf type.
L2 A leaf holding a clock time, a distance, or the name of a known entry fails at load.
L3 Rendered pages stay the same, except where typing corrects a real error.
D1 Part types, parsing and rendering.
D2 Load check for L2. after: D1
D3 Convert all of the trip file. Times are classified fact or plan; when that isn't clear, mark them unknown, visibly. after: D1
D4 Mentions of known entries become links, and backlinks come from the same data. after: D3
Out: new content, translation, editing tools, UI changes.

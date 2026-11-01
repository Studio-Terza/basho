# train-legs

Problem: a train leg without its platform, headsign, line and type sent the walker to the wrong train.

L1 A train leg cannot be constructed or loaded without platform or an explicit not-published marker, headsign, line and type.
L2 A trip file missing any of them fails at load.
L3 At a change, these are shown first.
D1 Types and the trip file format.
D2 Rendering at changes. after: D1
D3 Fill the existing trip's data from its sources, marking not published where it isn't known. after: D1
Out: other leg kinds, schedule lookups, UX beyond the leg card.

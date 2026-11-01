# verification

Problem: passing checks were read as correct, but data has different kinds of evidence.

L1 Every trip fact that can be wrong carries its evidence as a type: unverified, a named source, seen in simulator, confirmed by the user in use.
L2 Evidence cannot be added without saying what backs it.
L3 Unverified facts are visible to the reader, without colour.
D1 Evidence types and the trip file format, starting with lodging, trains and hours.
D2 Report: counts of facts by evidence kind, in the gate artifact. after: D1
D3 Existing data marked from what the research folder actually supports, and unverified otherwise. after: D1
D4 Reader marker for unverified facts. after: D1
Out: new fact kinds, automatic verification, web lookups.

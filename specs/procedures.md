# procedures

Problem: checks run by hand leave no trail and vary by agent.

L1 Each procedure is one Rust command that leaves an artifact: what ran, on which commit, the result.
L2 Procedures are Rust, not scripts.
D1 `cargo xtask gate`: finds Chromium, runs the full local gate, writes an artifact, and on a clean tree posts the `local-gate` commit status for that commit only if it passed.
D2 `cargo xtask sim`: release build in the simulator, drives the touch tests, checks crash reports by time, and writes an artifact. after: D1
Out: TestFlight, uploads, changes to app behaviour.

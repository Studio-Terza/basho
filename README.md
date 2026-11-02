# basho

An offline walking-trip guide. It is a Dioxus app (iOS and desktop) that shows whatever one trip file says: day-by-day legs, road by road, with times, places, a trip wiki, a map and a printable fallback. Nothing needs a network once the app is installed.

A small made-up trip, `data/trips/sample.toml`, is bundled. Load any other trip file from the app's Trip page, or pass its path as the first argument.

## Build

You need a recent stable Rust toolchain (see `rust-version` in `Cargo.toml`).

- Run the checks: `cargo xtask gate` (formatting, clippy, tests, lints, trip files, browser UI checks).
- Run the desktop app: `cd crates/app && dx serve --platform desktop`.
- Run on the iOS simulator (needs Xcode): `cd crates/app && dx serve --platform ios`.
- Print the days: `cargo xtask print`.

The shared lints come from [Studio-Terza/meta-dev](https://github.com/Studio-Terza/meta-dev): `cargo install --git https://github.com/Studio-Terza/meta-dev --rev 10eaa06225 --locked lint`.

## Trip file format

A trip file is one TOML file. The format is described in `specs/trip-file.md` and in `CLAUDE.md`; `cargo xtask trip-check <file>` lists the problems in a file.

## Map data

Map tiles are derived from OpenStreetMap data. (c) OpenStreetMap contributors, available under the [Open Database License (ODbL)](https://opendatacommons.org/licenses/odbl/1-0/). See <https://www.openstreetmap.org/copyright>. The app shows this credit on the map.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

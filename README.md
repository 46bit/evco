# jeepers

Incomplete Genetic Programming library for Rust.

## Status

[![Build Status](https://api.travis-ci.org/46bit/jeepers.svg)](https://travis-ci.org/46bit/jeepers) [![Coverage Status](https://coveralls.io/repos/github/46bit/jeepers/badge.svg)](https://coveralls.io/github/46bit/jeepers)

## Description

This approach is inspired by the GP in [Python's DEAP](https://github.com/DEAP/deap). The aim is for `jeepers` to be more performant and obtain simpler code through Rust's typesystem.

## Examples

`examples/snake.rs` will in time evolve a Snake AI. For now it simply generates a random tree. Run this using `cargo run --example snake`.

## Development

* Reformat code with `cargo fmt`.
* Lint code with `cargo build --features dev`.
* Run tests with `cargo test`.

## License

`jeepers` is distributed under the LGPLv3.0 license.

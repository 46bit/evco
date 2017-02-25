# `evco`

An Evolutionary Computing library for Rust. Currently an incomplete implementation of Genetic Programming.

## Status

[![Build Status](https://api.travis-ci.org/46bit/evco.svg)](https://travis-ci.org/46bit/evco) [![Coverage Status](https://coveralls.io/repos/github/46bit/evco/badge.svg)](https://coveralls.io/github/46bit/evco)

## Description

This approach is inspired by the GP in [Python's DEAP](https://github.com/DEAP/deap). The aim is for `evco` to be more performant and obtain simpler code through Rust's typesystem.

## Examples

`examples/snake.rs` will in time evolve a Snake AI. For now it simply evaluates random trees. Run this using `cargo run --example snake`.

## Development

* Reformat code with `cargo fmt`.
* Lint code with `cargo build --features dev`.
* Run tests with `cargo test`.

## License

`evco` is distributed under the LGPLv3.0 license.

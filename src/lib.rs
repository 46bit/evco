//! Genetic Programming for Rust
//!
//! This crate implements a Genetic Programming library inspired by the GP in
//! [Python's DEAP](https://github.com/DEAP/deap). This library is hoped to be
//! more performant and use Rust's typesystem to obtain simpler code.
//!
//! Presently under [active development](https://github.com/46bit/jeepers).

#![feature(box_syntax, associated_consts)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

extern crate rand;
// #[cfg(test)]
// extern crate quickcheck;

/// Module for generating Genetic Program trees.
pub mod tree;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

/// Types and utilities for Genetic Program trees.
pub mod tree;
mod crossover;
mod mutation;

/// Genetic Program crossover (mating).
pub use self::crossover::*;
/// Genetic Program mutation.
pub use self::mutation::*;

use rand::Rng;
use std::fmt;
use self::tree::*;

/// A genetic individual to mate and mutate in a Genetic Program.
///
/// Wraps around a `BoxTree` and caches useful data.
#[derive(Debug, Clone)]
pub struct Individual<T>
    where T: Tree
{
    /// The contained GP tree, starting at the head.
    pub tree: BoxTree<T>,
    nodes_count: usize,
}

impl<T> Individual<T>
    where T: Tree
{
    /// Generate a new Tree and individual.
    pub fn new<R: Rng>(tg: &mut TreeGen<R>) -> Individual<T> {
        Self::new_from_tree(T::tree(tg))
    }

    /// Create from a Tree.
    pub fn new_from_tree(boxtree: BoxTree<T>) -> Individual<T> {
        let mut indv = Individual {
            tree: boxtree,
            nodes_count: 0,
        };
        indv.recalculate_metadata();
        indv
    }

    /// Get cached number of nodes in tree.
    pub fn nodes_count(&self) -> usize {
        self.nodes_count
    }

    /// Update cached metadata such at the number of nodes in the tree.
    pub fn recalculate_metadata(&mut self) {
        self.nodes_count = self.tree.count_nodes();
    }
}

impl<T> fmt::Display for Individual<T>
    where T: Tree + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tree)
    }
}

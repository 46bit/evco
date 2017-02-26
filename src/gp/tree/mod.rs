mod gen;

pub use self::gen::*;

use rand::Rng;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;

/// Trait to be implemented by Genetic Programs trees.
pub trait Tree
    where Self: Sized + Debug + Clone
{
    /// Type of input when evaluating the tree.
    type Environment;

    /// The type the tree will evaluate to.
    type Action;

    /// Generate a new tree within the bounds specified by TreeGen.
    fn tree<R: Rng>(tg: &mut TreeGen<R>) -> BoxTree<Self> {
        Self::child(tg, 0)
    }

    /// Generate a random new node to go into a tree.
    fn child<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> BoxTree<Self> {
        if tg.have_reached_a_leaf(current_depth) {
            Self::leaf(tg, current_depth)
        } else {
            Self::branch(tg, current_depth)
        }
    }

    /// Generate a branch node (a node with at least one Tree child).
    fn branch<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> BoxTree<Self>;

    /// Generate a leaf node (a node without any Tree children).
    fn leaf<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> BoxTree<Self>;

    /// Count `Self` children of this node.
    fn count_children(&mut self) -> usize;

    /// Get children of this node.
    fn children(&self) -> Vec<&BoxTree<Self>>;

    /// Get mutable children of this node.
    fn children_mut(&mut self) -> Vec<&mut BoxTree<Self>>;

    /// Get indexed child of this node. Number children from 0; suggested to go left-to-right.
    //fn get_mut_child(&mut self, index: usize) -> Option<&mut BoxTree<Self>>;
    /// Used to evaluate the root node of a tree.
    fn evaluate(&self, env: &Self::Environment) -> Self::Action;
}

/// `Box` Wrapper for implementations of Tree.
#[derive(Clone, PartialEq, Eq)]
pub struct BoxTree<T>(Box<T>) where T: Tree;

impl<T> BoxTree<T>
    where T: Tree
{
    /// Extract the internal `Tree`.
    pub fn inner(self) -> T {
        *self.0
    }

    /// Count the number of nodes below this node in the tree.
    pub fn count_nodes(&mut self) -> usize {
        self.fold(0, |count, _, _, _| count + 1)
    }

    /// Get a clone of a particular value.
    pub fn get(&mut self, target_index: usize) -> Option<T> {
        let mut node = None;
        self.map_while(|current, index, _| if index == target_index {
            node = Some(current.clone());
            false
        } else {
            true
        });
        node
    }

    /// Traverse the tree with the ability to mutate nodes in-place.
    ///
    /// The callback receives a mutable pointer to the current node, the 0-based current iteration
    /// count, and the 0-based current depth in the tree.
    pub fn map<F>(&mut self, mut f: F)
        where F: FnMut(&mut T, usize, usize)
    {
        // @TOOD: Benchmark if this optimises out.
        self.map_while(|p, i, d| {
            f(p, i, d);
            true
        })
    }

    /// Traverse the tree until the function returns `false`.
    ///
    /// The callback receives a mutable pointer to the current node, the 0-based current iteration
    /// count, and the 0-based current depth in the tree.
    ///
    /// The callback returns a `bool`. If `false` the loop terminates.
    pub fn map_while<F>(&mut self, mut f: F)
        where F: FnMut(&mut T, usize, usize) -> bool
    {
        let mut stack: VecDeque<(&mut Self, usize)> = VecDeque::new();
        stack.push_back((self, 0));
        let mut i = 0;
        while let Some((node, depth)) = stack.pop_back() {
            if !f(node, i, depth) {
                break;
            }
            let mut children = node.children_mut();
            children.reverse();
            for child in children {
                stack.push_back((child, depth + 1));
            }
            i += 1;
        }
    }

    /// Traverse the tree building up a return value, with the ability to mutate nodes in-place.
    ///
    /// The callback receives the value being built up, a mutable pointer to the current node, the
    /// 0-based current iteration count, and the 0-based current depth in the tree.
    pub fn fold<F, V>(&mut self, value: V, mut f: F) -> V
        where F: FnMut(V, &mut T, usize, usize) -> V
    {
        // @TOOD: Benchmark if this optimises out.
        self.fold_while(value, |v, p, i, d| (true, f(v, p, i, d)))
    }

    /// Traverse the tree building up a return value, with the ability to mutate nodes in-place.
    ///
    /// The callback receives the value being built up, a mutable pointer to the current node, the
    /// 0-based current iteration count, and the 0-based current depth in the tree.
    ///
    /// The callback returns a pair of `(bool, fold_value)`. If `false` the loop terminates.
    pub fn fold_while<F, V>(&mut self, mut value: V, mut f: F) -> V
        where F: FnMut(V, &mut T, usize, usize) -> (bool, V)
    {
        let mut stack: VecDeque<(&mut Self, usize)> = VecDeque::new();
        stack.push_back((self, 0));
        let mut i = 0;
        while let Some((node, depth)) = stack.pop_back() {
            value = match f(value, node, i, depth) {
                (true, value) => value,
                (false, value) => return value,
            };
            let mut children = node.children_mut();
            children.reverse();
            for child in children {
                stack.push_back((child, depth + 1));
            }
            i += 1;
        }
        value
    }
}

/// Make `BoxTree` invisible in `Debug` output. At the cost of a little invisibility this
/// makes `Tree`s far more readable.
impl<T> Debug for BoxTree<T>
    where T: Tree
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> fmt::Display for BoxTree<T>
    where T: Tree + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Deref for BoxTree<T>
    where T: Tree
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for BoxTree<T>
    where T: Tree
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> From<T> for BoxTree<T>
    where T: Tree
{
    fn from(tree: T) -> BoxTree<T> {
        BoxTree(Box::new(tree))
    }
}

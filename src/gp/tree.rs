use gp::*;
use rand::Rng;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;

/// Newtype that wraps a boxed Tree element and implements helper methods.
#[derive(Clone)]
pub struct BoxTree<T>(Box<T>) where T: Tree;

impl<T> BoxTree<T>
    where T: Tree
{
    /// Iterate over every node of the tree with a `FnMut` or `TreeVisitor`.
    pub fn visit<'b, V>(&self, visitor: &mut V)
        where V: 'b + TreeVisitor<T>
    {
        let mut stack: VecDeque<&Self> = VecDeque::new();
        stack.push_back(self);
        while let Some(node) = stack.pop_back() {
            visitor.visit_node(node);
            let mut children = node.children();
            children.reverse();
            for child in children {
                stack.push_back(child);
            }
        }
    }

    /// Iterate and mutate over every node of the tree with a `FnMut` or `TreeVisitor`.
    pub fn visit_mut<'b, V>(&mut self, visitor: &mut V)
        where V: 'b + TreeMutVisitor<T>
    {
        let mut stack: VecDeque<&mut Self> = VecDeque::new();
        stack.push_back(self);
        while let Some(node) = stack.pop_back() {
            visitor.visit_mut_node(node);
            let mut children = node.children_mut();
            children.reverse();
            for child in children {
                stack.push_back(child);
            }
        }
    }

    /// Count the number of nodes below this node in the tree.
    fn count_nodes(&mut self) -> usize {
        let mut count = 0;
        self.visit(&mut |_: &T| count += 1);
        count
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

/// For running a callback against every node of a tree. Use with `Tree::visit`.
///
/// Suggested uses: counting nodes in a tree, depth of a tree, etc.
pub trait TreeVisitor<T> {
    /// Callback to run with each node.
    fn visit_node(&mut self, node: &T);
}

impl<F, T> TreeVisitor<T> for F
    where F: FnMut(&T)
{
    fn visit_node(&mut self, node: &T) {
        self(node)
    }
}

/// For running a callback against every node of a tree. Use with `Tree::visit`.
///
/// Suggested uses: counting nodes in a tree, depth of a tree, etc.
pub trait TreeMutVisitor<T> {
    /// Callback to run with each node.
    fn visit_mut_node(&mut self, node: &mut T);
}

impl<F, T> TreeMutVisitor<T> for F
    where F: FnMut(&mut T)
{
    fn visit_mut_node(&mut self, node: &mut T) {
        self(node)
    }
}

/// The Tree trait will be implemented by the trees of all Genetic Programs.
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
    fn evaluate(&self, env: Self::Environment) -> Self::Action;
}

/// Wrapper around complete Genetic Program trees, caching useful data and indicating
/// the head of the tree..
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

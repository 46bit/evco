use rand::Rng;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;

/// Wrapper around complete Genetic Program trees, caching useful data and indicating
/// the head of the tree.
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
    fn evaluate(&self, env: &Self::Environment) -> Self::Action;
}

/// Newtype that wraps a boxed Tree element and implements helper methods.
#[derive(Clone)]
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

/// The tree generation mode in use. See `TreeGen`.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum TreeGenMode {
    /// Corresponds to `TreeGen::perfect`.
    Perfect(usize),
    /// Corresponds to `TreeGen::full`.
    Full,
    /// Corresponds to `TreeGen::full_ranged`.
    FullRanged(usize),
}

/// Configure generation of trees. This manages tree depth by deciding when to
/// generate a Terminal (leaf) node.
#[derive(PartialEq, Eq, Debug)]
pub struct TreeGen<'a, R>
    where R: 'a + Rng
{
    /// Which tree depth logic to use.
    mode: TreeGenMode,
    /// A `rand::Rng` implementation for generating random tree nodes.
    rng: &'a mut R,
    /// The minimum depth of trees to generate.
    min_depth: usize,
    /// The maximum depth of trees to generate.
    max_depth: usize,
}

impl<'a, R> TreeGen<'a, R>
    where R: 'a + Rng
{
    /// Generate a perfect tree. All leaves are at the same depth in the range
    /// [min_depth, max_depth].
    ///
    /// **This is the equivalent of DEAP's `genFull`.**
    pub fn perfect(rng: &mut R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        let chosen_depth = rng.gen_range(min_depth, max_depth + 1);
        TreeGen {
            rng: rng,
            mode: TreeGenMode::Perfect(chosen_depth),
            min_depth: min_depth,
            max_depth: max_depth,
        }
    }

    /// Generate a full tree, one with leaves at varying depths. Leaf depths are
    /// linearly distributed between min_depth and a chosen depth in the range.
    ///
    /// **This is NOT the same as DEAP's `genFull`. See `TreeGen::full`**
    pub fn full(rng: &mut R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        TreeGen {
            rng: rng,
            mode: TreeGenMode::Full,
            min_depth: min_depth,
            max_depth: max_depth,
        }
    }

    /// Generate a full tree, one with leaves at varying depths. Leaf depths are
    /// linearly distributed between min_depth and a chosen depth in the range.
    ///
    /// **This is the equivalent of DEAP's `genGrow`.**
    pub fn full_ranged(rng: &mut R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        let chosen_depth = rng.gen_range(min_depth, max_depth + 1);
        TreeGen {
            rng: rng,
            mode: TreeGenMode::FullRanged(chosen_depth),
            min_depth: min_depth,
            max_depth: max_depth,
        }
    }

    /// Randomly choose between `TreeGen::perfect` and `TreeGen::full_ranged`.
    ///
    /// **This is the equivalent of DEAP's `genHalfAndHalf`.**
    // N.B. If TreeGen is ever Clone the random choice needs revising.
    pub fn half_and_half(rng: &mut R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        if rng.gen() {
            Self::perfect(rng, min_depth, max_depth)
        } else {
            Self::full_ranged(rng, min_depth, max_depth)
        }
    }

    /// Chooses whether to generate a Leaf node. Used by `Tree::child`.
    pub fn have_reached_a_leaf(&mut self, current_depth: usize) -> bool {
        match self.mode {
            TreeGenMode::Perfect(chosen_depth) => current_depth == chosen_depth,
            TreeGenMode::Full => {
                // This given an equal 1-in-depth_interval chance at every intermediary depth.
                // Earlier checks ensure in the (1/depth)*(depth-1) case we reach chosen_depth,
                // we do finally place a Leaf.
                let depth_interval = self.max_depth - self.min_depth;
                // @TODO: Avoid converting depth_interval.
                current_depth == self.max_depth ||
                (current_depth >= self.min_depth) && self.gen_weighted_bool(depth_interval as u32)
            }
            TreeGenMode::FullRanged(chosen_depth) => {
                // This given an equal 1-in-depth_interval chance at every intermediary depth.
                // Earlier checks ensure in the (1/depth)*(depth-1) case we reach chosen_depth,
                // we do finally place a Leaf.
                let depth_interval = chosen_depth - self.min_depth;
                // @TODO: Avoid converting depth_interval.
                current_depth == chosen_depth ||
                (current_depth >= self.min_depth) && self.gen_weighted_bool(depth_interval as u32)
            }
        }
    }
}

impl<'a, R> Rng for TreeGen<'a, R>
    where R: 'a + Rng
{
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    // some RNGs implement these more efficiently than the default, so
    // we might as well defer to them.
    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }
}

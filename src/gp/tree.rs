use rand::Rng;

/// The Tree trait will be implemented by the trees of all Genetic Programs.
pub trait Tree<'a>
    where Self: Sized
{
    /// Type of input when evaluating the tree.
    type Environment;

    /// The type the tree will evaluate to.
    type Action;

    /// Generate a new tree within the bounds specified by TreeGen.
    fn tree<R: Rng>(tg: &mut TreeGen<R>) -> Self {
        Self::child(tg, 0)
    }

    /// Generate a random new node to go into a tree.
    fn child<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self {
        if tg.have_reached_a_leaf(current_depth) {
            Self::leaf(tg, current_depth)
        } else {
            Self::branch(tg, current_depth)
        }
    }

    /// Generate a branch node (a node with at least one Tree child).
    fn branch<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self;

    /// Generate a leaf node (a node without any Tree children).
    fn leaf<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self;

    /// Used to evaluate the root node of a Tree.
    fn evaluate(&self, env: Self::Environment) -> Self::Action;

    /// For calling TreeVisitor::visit_node on every node of a Tree.
    fn visit<'b, V>(&self, visitor: &mut V)
        where V: TreeVisitor<'a, 'b, Self>,
              Self: 'b;
}

/// The tree generation mode in use. See `TreeGen`.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TreeGenMode {
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
    pub mode: TreeGenMode,
    /// A `rand::Rng` implementation for generating random tree nodes.
    pub rng: &'a mut R,
    /// The minimum depth of trees to generate.
    pub min_depth: usize,
    /// The maximum depth of trees to generate.
    pub max_depth: usize,
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

/// For running a callback against every node of a tree. Use with `Tree::visit`.
///
/// Suggested uses: counting nodes in a tree, depth of a tree, etc.
pub trait TreeVisitor<'a, 'b, T>
    where T: 'b + Tree<'a>
{
    /// Callback to run with each node.
    fn visit_node(&mut self, node: &T);
}

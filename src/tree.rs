use rand::Rng;

/// The Tree trait will be implemented by the trees of all Genetic Programs.
///
/// Generally only `rand_terminal` and `rand_nonterminal` will need redefining
/// as other methods have default implementations.
pub trait Tree<'a>
    where Self: Sized
{
    /// Type of input when evaluating the tree.
    type Environment;

    /// The type the tree will evaluate to.
    type Action;

    /// Generate a new tree within the bounds specified by TreeGen.
    fn rand_tree<R: Rng>(tg: &mut TreeGen<R>) -> Self {
        Self::rand_node(tg, 0)
    }

    /// Generate a random new node to go into a tree.
    fn rand_node<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self {
        let terminal_proportion = Self::terminal_proportion(tg);
        if tg.condition(current_depth, terminal_proportion) {
            Self::rand_terminal(tg, current_depth)
        } else {
            Self::rand_nonterminal(tg, current_depth)
        }
    }

    /// Generate a Terminal node (a leaf) to go into a tree.
    fn rand_terminal<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self;

    /// Generate a Non-Terminal node to go into a tree.
    fn rand_nonterminal<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self;

    /// What proportion of possible tree nodes are terminals? 0.0 to 1.0.
    /// TreeGen argument may be removed.
    fn terminal_proportion<R: Rng>(tg: &mut TreeGen<R>) -> f32;

    /// `evaluate` is called on the root node of a Tree to get its output.
    fn evaluate(&self, env: Self::Environment) -> Self::Action;
}

/// Whether we're generating fully-balanced trees, or ones whose leaves are at
/// varying lengths. Generally only useful inside Tree implementations - use
/// `TreeGen::full`, `TreeGen::grow`, `TreeGen::half_and_half`.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TreeGenMode {
    /// Generating a fully-balanced tree with Terminals at a fixed depth.
    Full,
    /// Generating a tree with Terminals (leaves) at varying depths.
    Grow,
}

/// Configure generation of trees. This manages tree depth by deciding when to
/// generate a Terminal (leaf) node.
#[derive(PartialEq, Eq, Debug)]
pub struct TreeGen<'a, R>
    where R: 'a + Rng
{
    /// A `rand::Rng` implementation for generating random tree nodes.
    pub rng: &'a mut R,
    /// Which tree depth logic to use.
    pub mode: TreeGenMode,
    /// The minimum depth of trees to generate.
    pub min_depth: usize,
    /// The maximum depth of trees to generate.
    pub max_depth: usize,
    /// Internal randomly-chosen height to make the tree.
    pub chosen_depth: usize,
}

impl<'a, R> TreeGen<'a, R>
    where R: 'a + Rng
{
    /// Generate a fully-balanced tree between the depth bounds.
    pub fn full(rng: &mut R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        let chosen_depth = rng.gen_range(min_depth, max_depth + 1);
        TreeGen {
            rng: rng,
            mode: TreeGenMode::Full,
            min_depth: min_depth,
            max_depth: max_depth,
            chosen_depth: chosen_depth,
        }
    }

    /// Generate a varying-depth tree between the depth bounds.
    pub fn grow(rng: &mut R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        let chosen_depth = rng.gen_range(min_depth, max_depth + 1);
        TreeGen {
            rng: rng,
            mode: TreeGenMode::Grow,
            min_depth: min_depth,
            max_depth: max_depth,
            chosen_depth: chosen_depth,
        }
    }

    /// Randomly choose between a fully-balanced tree and a varying-depth tree.
    // N.B. If TreeGen is ever Clone the random choice needs revising.
    pub fn half_and_half(rng: &mut R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        if rng.gen() {
            Self::full(rng, min_depth, max_depth)
        } else {
            Self::grow(rng, min_depth, max_depth)
        }
    }

    /// Chooses whether to generate a Terminal (leaf) node. Used by `Tree::rand_node`.
    pub fn condition(&mut self, current_depth: usize, term_prob: f32) -> bool {
        match self.mode {
            TreeGenMode::Full => current_depth == self.chosen_depth,
            TreeGenMode::Grow => {
                (current_depth == self.chosen_depth) ||
                (current_depth >= self.min_depth && self.next_f32() < term_prob)
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

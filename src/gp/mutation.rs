use gp::*;
use rand::Rng;

/// The mutation mode in use. See `Mutation`.
#[derive(PartialEq, Clone, Copy, Debug)]
enum MutationMode {
    /// Corresponds to `Mutation::shrink`.
    Shrink,
    /// Corresponds to `Mutation::uniform`.
    Uniform,
    /// Corresponds to `Mutation::node_replacement`.
    NodeReplacement,
    /// Corresponds to `Mutation::ephemeral_one` and `Mutation::ephemeral_all`.
    Ephemeral(EphemeralMode),
    /// Corresponds to `Mutation::insert`.
    Insert,
}

/// Modes of ephemeral mutation. See `MutationMode::Ephemeral`.
#[derive(PartialEq, Clone, Copy, Debug)]
enum EphemeralMode {
    One,
    All,
}

/// Performs mutation on an individual.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Mutation {
    mode: MutationMode,
}

impl Mutation {
    /// Perform mutation by randomly replacing a node with one of its children.
    #[doc(hidden)]
    pub fn shrink() -> Mutation {
        Mutation { mode: MutationMode::Shrink }
    }

    /// Perform mutation by randomly replacing a node with a new subtree.
    pub fn uniform() -> Mutation {
        Mutation { mode: MutationMode::Uniform }
    }

    /// Perform mutation by randomly replacing a node with a new subtree. The replacement
    /// node will have the same number of children.
    #[doc(hidden)]
    pub fn node_replacement() -> Mutation {
        Mutation { mode: MutationMode::NodeReplacement }
    }

    /// Randomly replace a constant value in the tree with another value.
    #[doc(hidden)]
    pub fn ephemeral_one() -> Mutation {
        Mutation { mode: MutationMode::Ephemeral(EphemeralMode::One) }
    }

    /// Randomly replace all constant values in the tree.
    #[doc(hidden)]
    pub fn ephemeral_all() -> Mutation {
        Mutation { mode: MutationMode::Ephemeral(EphemeralMode::All) }
    }

    /// Insert a new node at a randomly chosen position. The existing node and its children
    /// will be a child of the new node.
    #[doc(hidden)]
    pub fn insert() -> Mutation {
        Mutation { mode: MutationMode::Insert }
    }

    /// Mutate an individual according to the configured mutation mode.
    pub fn mutate<T, R>(&self, indv: &mut Individual<T>, tg: &mut TreeGen<R>)
        where T: Tree,
              R: Rng
    {
        match self.mode {
            MutationMode::Shrink => self.mutate_shrink(indv, tg),
            MutationMode::Uniform => self.mutate_uniform(indv, tg),
            MutationMode::NodeReplacement => self.mutate_node_replacement(indv, tg),
            MutationMode::Ephemeral(EphemeralMode::One) => self.mutate_ephemeral_one(indv, tg),
            MutationMode::Ephemeral(EphemeralMode::All) => self.mutate_ephemeral_all(indv, tg),
            MutationMode::Insert => self.mutate_insert(indv, tg),
        }
    }

    fn mutate_shrink<T, R>(&self, _: &mut Individual<T>, _: &mut TreeGen<R>)
        where T: Tree,
              R: Rng
    {
        unimplemented!();
    }

    fn mutate_uniform<T, R>(&self, indv: &mut Individual<T>, tg: &mut TreeGen<R>)
        where T: Tree,
              R: Rng
    {
        let target_index = tg.gen_range(0, indv.nodes_count());
        indv.tree.map_while(|node, index, _| if index == target_index {
            *node = T::tree(tg).inner();
            false
        } else {
            true
        });
    }

    fn mutate_node_replacement<T, R>(&self, _: &mut Individual<T>, _: &mut TreeGen<R>)
        where T: Tree,
              R: Rng
    {
        unimplemented!();
    }

    fn mutate_ephemeral_one<T, R>(&self, _: &mut Individual<T>, _: &mut TreeGen<R>)
        where T: Tree,
              R: Rng
    {
        unimplemented!();
    }

    fn mutate_ephemeral_all<T, R>(&self, _: &mut Individual<T>, _: &mut TreeGen<R>)
        where T: Tree,
              R: Rng
    {
        unimplemented!();
    }

    fn mutate_insert<T, R>(&self, _: &mut Individual<T>, _: &mut TreeGen<R>)
        where T: Tree,
              R: Rng
    {
        unimplemented!();
    }
}

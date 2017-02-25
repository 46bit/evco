use gp::*;
use rand::Rng;

/// The crossover mode in use. See `Crossover`.
#[derive(PartialEq, Clone, Copy, Debug)]
enum CrossoverMode {
    /// Corresponds to `Crossover::one_point`.
    OnePoint,
    /// Corresponds to `TreeGen::one_point_leaf_biased`.
    OnePointLeafBiased(f32),
}

/// Performs crossover between individuals.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Crossover {
    mode: CrossoverMode,
}

impl Crossover {
    /// Get an operator to perform one-point crossover between two individuals.
    ///
    /// The subtree at a random position in one individual will be swapped with a random
    /// position in a second individual.
    pub fn one_point() -> Crossover {
        Crossover { mode: CrossoverMode::OnePoint }
    }

    /// Get an operator to perform one-point crossover between two individuals.
    ///
    /// The subtree at a random position in one individual will be swapped with a random
    /// position in a second individual. Each swap points will be a terminal with `termpb`
    /// probability.
    pub fn one_point_leaf_biased(termpb: f32) -> Crossover {
        Crossover { mode: CrossoverMode::OnePointLeafBiased(termpb) }
    }

    /// Perform crossover between two individuals according to the crossover mode created with.
    pub fn mate<T, R>(&self, indv1: &mut Individual<T>, indv2: &mut Individual<T>, rng: R)
        where T: Tree,
              R: Rng
    {
        match self.mode {
            CrossoverMode::OnePoint => self.mate_one_point(indv1, indv2, rng),
            CrossoverMode::OnePointLeafBiased(termpb) => {
                self.mate_one_point_leaf_biased(indv1, indv2, termpb, rng)
            }
        }
    }

    fn mate_one_point<T: Tree, R: Rng>(&self,
                                       indv1: &mut Individual<T>,
                                       indv2: &mut Individual<T>,
                                       mut rng: R)
        where T: Tree,
              R: Rng
    {
        let index1 = rng.gen_range(0, indv1.nodes_count());
        let mut node_from_1 = None;
        let mut count = 0;
        indv1.tree.visit(&mut |node: &T| {
            if count == index1 {
                node_from_1 = Some(node.clone());
            }
            count += 1;
        });
        let node_from_1 = node_from_1.expect("No node selected from indv2.");

        let index2 = rng.gen_range(0, indv2.nodes_count());
        let mut node_from_2 = None;
        count = 0;
        indv2.tree.visit(&mut |node: &T| {
            if count == index2 {
                node_from_2 = Some(node.clone());
            }
            count += 1;
        });
        let node_from_2 = node_from_2.expect("No node selected from indv1.");

        count = 0;
        indv1.tree.visit_mut(&mut |node: &mut T| {
            if count == index1 {
                *node = node_from_2.clone();
            }
            count += 1;
        });

        count = 0;
        indv2.tree.visit_mut(&mut |node: &mut T| {
            if count == index2 {
                *node = node_from_1.clone();
            }
            count += 1;
        });

        indv1.recalculate_metadata();
        indv2.recalculate_metadata();
    }

    fn mate_one_point_leaf_biased<T, R>(&self,
                                        _: &mut Individual<T>,
                                        _: &mut Individual<T>,
                                        _: f32,
                                        _: R)
        where T: Tree,
              R: Rng
    {
        unimplemented!();
    }
}

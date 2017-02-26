use gp::*;
use std::mem;
use rand::Rng;

/// The crossover mode in use. See `Crossover`.
#[derive(PartialEq, Clone, Copy, Debug)]
enum CrossoverMode {
    /// Corresponds to `Crossover::one_point`.
    OnePoint,
    /// Corresponds to `Crossover::one_point_leaf_biased`.
    OnePointLeafBiased(f32),
}

/// Configures crossover (mating) between GP individuals.
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
    #[doc(hidden)]
    pub fn one_point_leaf_biased(termpb: f32) -> Crossover {
        Crossover { mode: CrossoverMode::OnePointLeafBiased(termpb) }
    }

    /// Crossover (mate) two individuals according to the configured crossover mode.
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
        let target_index1 = rng.gen_range(0, indv1.nodes_count());
        let target_index2 = rng.gen_range(0, indv2.nodes_count());

        indv1.tree.map_while(|node1, index1, _| if index1 == target_index1 {
            indv2.tree.map_while(|node2, index2, _| if index2 == target_index2 {
                mem::swap(node1, node2);
                false
            } else {
                true
            });
            false
        } else {
            true
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

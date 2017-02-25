use gp::tree::*;

/// The crossover mode in use. See `Crossover`.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CrossoverMode {
    /// Corresponds to `Crossover::one_point`.
    OnePoint,
    /// Corresponds to `TreeGen::one_point_leaf_biased`.
    OnePointLeafBiased(f32),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Crossover {
    mode: CrossoverMode,
}

impl Crossover {
    pub fn one_point() -> Crossover {
        Crossover { mode: CrossoverMode::OnePoint }
    }

    pub fn one_point_leaf_biased(termpb: f32) -> Crossover {
        Crossover { mode: CrossoverMode::OnePointLeafBiased(termpb) }
    }

    pub fn mate<T>(&self, indv1: &mut T, indv2: &mut T)
        where T: Tree
    {
        match self.mode {
            CrossoverMode::OnePoint => self.mate_one_point(indv1, indv2),
            CrossoverMode::OnePointLeafBiased(termpb) => {
                self.mate_one_point_leaf_biased(indv1, indv2, termpb)
            }
        }
    }

    fn mate_one_point<T: Tree>(&self, indv1: &mut T, indv2: &mut T) {
        let counter = NodeCounter::new();
        let indv1_node_count = indv1.visit(&mut counter);
        let counter = NodeCounter::new();
        let indv2_node_count = indv2.visit(&mut counter);

        let index1 = rng.gen_range(0, indv1.nodes_count);
        let nodeFrom1 = None;
        let count1 = 0;
        indv1.tree.visit(&mut |node: &T| {
            if count1 == index1 {
                nodeFrom1 = Some(node.clone());
            }
            count1 += 1;
        });

        let index2 = rng.gen_range(0, indv2.nodes_count);
        let nodeFrom2 = None;
        let count2 = 0;
        indv2.tree.visit(&mut |node: &T| {
            if count2 == index2 {
                nodeFrom2 = Some(node.clone());
            }
            count2 += 1;
        });

        let count1 = 0;
        indv1.tree.visit_mut(&mut |node: &mut T| {
            if count1 == index1 {
                *node = nodeFrom2;
            }
            count1 += 1;
        });

        let count2 = 0;
        indv2.tree.visit_mut(&mut |node: &mut T| {
            if count2 == index2 {
                *node = nodeFrom1;
            }
            count2 += 1;
        });

        indv1.recalculate_metadata();
        indv2.recalculate_metadata();
    }

    fn mate_one_point_leaf_biased<T: Tree>(&self, indv1: &mut T, indv2: &mut T, termpb: f32) {
        unimplemented!();
    }
}

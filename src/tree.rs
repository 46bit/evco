use quickcheck::Gen;

pub trait Tree
    where Self: Sized
{
    const TERMINAL_PROPORTION: f32;

    fn arbitrary_tree<G: Gen>(tg: &mut TreeGen<G>) -> Self {
        Self::arbitrary_node(tg, 0)
    }

    fn arbitrary_node<G: Gen>(tg: &mut TreeGen<G>, current_depth: usize) -> Self {
        if tg.condition(current_depth, Self::TERMINAL_PROPORTION) {
            Self::arbitrary_terminal(tg, current_depth)
        } else {
            Self::arbitrary_nonterminal(tg, current_depth)
        }
    }

    fn arbitrary_terminal<G: Gen>(tg: &mut TreeGen<G>, current_depth: usize) -> Self;

    fn arbitrary_nonterminal<G: Gen>(tg: &mut TreeGen<G>, current_depth: usize) -> Self;
}

#[derive(PartialEq, Eq, Debug)]
pub enum TreeGenMode {
    Full,
    Grow,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TreeGen<'a, G: Gen> where G: 'a {
    pub gen: &'a mut G,
    pub mode: TreeGenMode,
    pub min_depth: usize,
    pub max_depth: usize,
    pub chosen_depth: usize,
}

impl<'a, G> TreeGen<'a, G> where G: Gen {
    pub fn full(gen: &mut G, min_depth: usize, max_depth: usize) -> TreeGen<G> {
        let chosen_depth = gen.gen_range(min_depth, max_depth + 1);
        TreeGen {
            gen: gen,
            mode: TreeGenMode::Full,
            min_depth: min_depth,
            max_depth: max_depth,
            chosen_depth: chosen_depth,
        }
    }

    pub fn grow(gen: &mut G, min_depth: usize, max_depth: usize) -> TreeGen<G> {
        let chosen_depth = gen.gen_range(min_depth, max_depth + 1);
        TreeGen {
            gen: gen,
            mode: TreeGenMode::Grow,
            min_depth: min_depth,
            max_depth: max_depth,
            chosen_depth: chosen_depth,
        }
    }

    pub fn half_and_half(gen: &mut G, min_depth: usize, max_depth: usize) -> TreeGen<G> {
        let chosen_depth = gen.gen_range(min_depth, max_depth + 1);
        let mode = match gen.next_u32() % 2 {
            0 => TreeGenMode::Full,
            1 => TreeGenMode::Grow,
            _ => unreachable!(),
        };
        TreeGen {
            gen: gen,
            mode: mode,
            min_depth: min_depth,
            max_depth: max_depth,
            chosen_depth: chosen_depth,
        }
    }

    pub fn condition(&mut self, current_depth: usize, term_prob: f32) -> bool {
        match self.mode {
            TreeGenMode::Full => current_depth == self.chosen_depth,
            TreeGenMode::Grow => {
                (current_depth == self.chosen_depth) ||
                (current_depth >= self.min_depth && self.gen.next_f32() < term_prob)
            }
        }
    }
}

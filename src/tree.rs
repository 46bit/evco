use quickcheck::Gen;

pub trait Tree
    where Self: Sized
{
    const TERMINAL_PROPORTION: f32;

    fn arbitrary_tree<G: Gen>(g: &mut G, t: TreeGen) -> Self {
        let chosen_t_ = t.choose_depth(g);
        Self::arbitrary_node(g, &chosen_t_, 0)
    }

    fn arbitrary_node<G: Gen>(g: &mut G, t: &TreeGen, depth: usize) -> Self {
        if t.condition(g, depth, Self::TERMINAL_PROPORTION) {
            Self::arbitrary_terminal(g, t, depth)
        } else {
            Self::arbitrary_nonterminal(g, t, depth)
        }
    }

    fn arbitrary_terminal<G: Gen>(g: &mut G, t: &TreeGen, depth: usize) -> Self;

    fn arbitrary_nonterminal<G: Gen>(g: &mut G, t: &TreeGen, depth: usize) -> Self;
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum TreeGenMode {
    Full,
    Grow,
    HalfAndHalf,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct TreeGen {
    pub mode: TreeGenMode,
    pub min_depth: usize,
    pub max_depth: usize,
    pub chosen_depth: Option<usize>,
}

impl TreeGen {
    pub fn full(min_depth: usize, max_depth: usize) -> TreeGen {
        TreeGen {
            mode: TreeGenMode::Full,
            min_depth: min_depth,
            max_depth: max_depth,
            chosen_depth: None,
        }
    }

    pub fn grow(min_depth: usize, max_depth: usize) -> TreeGen {
        TreeGen {
            mode: TreeGenMode::Grow,
            min_depth: min_depth,
            max_depth: max_depth,
            chosen_depth: None,
        }
    }

    fn choose_depth<G: Gen>(mut self, g: &mut G) -> TreeGen {
        self.chosen_depth = Some(g.gen_range(self.min_depth, self.max_depth + 1));
        if self.mode == TreeGenMode::HalfAndHalf {
            self.mode = match g.next_u32() % 2 {
                0 => TreeGenMode::Full,
                1 => TreeGenMode::Grow,
                _ => unreachable!(),
            };
        }
        self
    }

    fn condition<G: Gen>(&self, g: &mut G, depth: usize, term_prob: f32) -> bool {
        let chosen_depth = self.chosen_depth.expect("No chosen depth set!");
        match self.mode {
            TreeGenMode::Full => depth == chosen_depth,
            TreeGenMode::Grow => {
                (depth == chosen_depth) || (depth >= self.min_depth && g.next_f32() < term_prob)
            }
            TreeGenMode::HalfAndHalf => unreachable!("HalfAndHalf is decided in chosen_depth."),
        }
    }
}

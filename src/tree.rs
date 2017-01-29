use rand::Rng;

pub trait Tree
    where Self: Sized
{
    const TERMINAL_PROPORTION: f32;

    fn rand_tree<R: Rng>(tg: &mut TreeGen<R>) -> Self {
        Self::rand_node(tg, 0)
    }

    fn rand_node<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self {
        if tg.condition(current_depth, Self::TERMINAL_PROPORTION) {
            Self::rand_terminal(tg, current_depth)
        } else {
            Self::rand_nonterminal(tg, current_depth)
        }
    }

    fn rand_terminal<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self;

    fn rand_nonterminal<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Self;
}

#[derive(PartialEq, Eq, Debug)]
pub enum TreeGenMode {
    Full,
    Grow,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TreeGen<'a, R>
    where R: 'a + Rng
{
    pub rng: &'a mut R,
    pub mode: TreeGenMode,
    pub min_depth: usize,
    pub max_depth: usize,
    pub chosen_depth: usize,
}

impl<'a, R> TreeGen<'a, R>
    where R: 'a + Rng
{
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

    pub fn half_and_half(rng: &mut R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        match rng.gen() {
            true => Self::full(rng, min_depth, max_depth),
            false => Self::grow(rng, min_depth, max_depth),
        }
    }

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

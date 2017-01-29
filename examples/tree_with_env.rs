extern crate rand;
extern crate jeepers;

use rand::{OsRng, Rng, Rand};

use jeepers::tree::{Tree, TreeGen};

#[derive(Clone, Debug)]
pub enum DemoTree {
    A(Box<DemoTree>),
    B(usize),
}

impl<'a> Tree<'a> for DemoTree {
    type Environment = bool;
    type Action = usize;

    fn terminal_proportion<R: Rng>(_: &mut TreeGen<R>) -> f32 {
        0.5
    }

    fn rand_terminal<R: Rng>(tg: &mut TreeGen<R>, _: usize) -> DemoTree {
        DemoTree::B(usize::rand(tg))
    }

    fn rand_nonterminal<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> DemoTree {
        DemoTree::A(Box::new(DemoTree::rand_node(tg, current_depth + 1)))
    }

    fn evaluate(&self, env: bool) -> Self::Action {
        match *self {
            DemoTree::A(ref child) => child.evaluate(env) + 1,
            DemoTree::B(n) => {
                if env {
                    n
                } else {
                    0
                }
            },
        }
    }
}

fn main() {
    let mut rng = OsRng::new().unwrap();
    let mut tree_gen = TreeGen::full(&mut rng, 1, 4);

    let tree = DemoTree::rand_tree(&mut tree_gen);
    println!("{:?}", tree);
    println!("{:?}", tree.evaluate(true));
    println!("{:?}", tree.evaluate(false));
}

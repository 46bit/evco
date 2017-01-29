extern crate rand;
extern crate jeepers;

use rand::{OsRng, Rng, Rand};

use jeepers::tree::{Tree, TreeGen};

#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Ahead,
    Right,
}

impl Rand for Direction {
    fn rand<R: Rng>(r: &mut R) -> Direction {
        match r.next_u32() % 3 {
            0 => Direction::Left,
            1 => Direction::Ahead,
            2 => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SnakeTree {
    IfDanger(Direction, Box<SnakeTree>, Box<SnakeTree>),
    IfFood(Direction, Box<SnakeTree>, Box<SnakeTree>),
    Move(Direction),
}

impl Tree for SnakeTree {
    fn terminal_proportion<R: Rng>(_: &mut TreeGen<R>) -> f32 {
        0.1 / (0.1 + 0.2)
    }

    fn rand_terminal<R: Rng>(tg: &mut TreeGen<R>, _: usize) -> SnakeTree {
        SnakeTree::Move(Direction::rand(tg))
    }

    fn rand_nonterminal<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> SnakeTree {
        // A list of nonterminal construction methods.
        let nonterminal_fs = [SnakeTree::rand_if_danger, SnakeTree::rand_if_food];
        // Picks a random nonterminal constructor and runs it.
        let nonterminal_f = tg.choose(&nonterminal_fs).unwrap();
        nonterminal_f(tg, current_depth)
    }
}

impl SnakeTree {
    fn rand_if_danger<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> SnakeTree {
        let direction = Direction::rand(tg);
        let true_ = SnakeTree::rand_node(tg, current_depth + 1);
        let false_ = SnakeTree::rand_node(tg, current_depth + 1);
        SnakeTree::IfDanger(direction, Box::new(true_), Box::new(false_))
    }

    fn rand_if_food<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> SnakeTree {
        let direction = Direction::rand(tg);
        let true_ = SnakeTree::rand_node(tg, current_depth + 1);
        let false_ = SnakeTree::rand_node(tg, current_depth + 1);
        SnakeTree::IfFood(direction, Box::new(true_), Box::new(false_))
    }
}

fn main() {
    let mut rng = OsRng::new().unwrap();

    let d = Direction::rand(&mut rng);
    println!("{:?}", d);

    let mut tree_gen = TreeGen::full(&mut rng, 5, 10);
    let p = SnakeTree::rand_tree(&mut tree_gen);
    println!("{:?}", p);
}

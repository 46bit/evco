extern crate rand;
extern crate jeepers;

use rand::{OsRng, Rand};

use jeepers::tree::{Tree, TreeGen};
use jeepers::snake::{Primitive, Direction};

fn main() {
    let mut rng = OsRng::new().unwrap();

    let d = Direction::rand(&mut rng);
    println!("{:?}", d);

    let mut tree_gen = TreeGen::full(&mut rng, 5, 10);
    let p = Primitive::rand_tree(&mut tree_gen);
    println!("{:?}", p);
}

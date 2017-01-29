extern crate rand;
extern crate quickcheck;
extern crate jeepers;

use quickcheck::{StdGen, Arbitrary};
use rand::OsRng;

use jeepers::tree::{Tree, TreeGen};
use jeepers::snake::{Primitive, Direction};

fn main() {
    let mut gen = StdGen::new(OsRng::new().unwrap(), 50);

    let d = Direction::arbitrary(&mut gen);
    println!("{:?}", d);

    let tree_gen = TreeGen::full(5, 10);
    let p = Primitive::arbitrary_tree(&mut gen, tree_gen);
    println!("{:?}", p);
}

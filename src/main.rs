extern crate rand;
extern crate quickcheck;
extern crate jeepers;

use quickcheck::{StdGen, Arbitrary};
use rand::OsRng;
use jeepers::snake;

fn main() {
    let mut gen = StdGen::new(OsRng::new().unwrap(), 50);

    let d = snake::Direction::arbitrary(&mut gen);
    println!("{:?}", d);

    let t = snake::Primitive::arbitrary(&mut gen);
    println!("{:?}", t);
}

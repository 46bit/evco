use quickcheck::{Gen, Arbitrary};

use tree::*;

#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Ahead,
    Right,
}

impl Arbitrary for Direction {
    fn arbitrary<G: Gen>(g: &mut G) -> Direction {
        match g.next_u32() % 3 {
            0 => Direction::Left,
            1 => Direction::Ahead,
            2 => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Primitive {
    IfDanger(Direction, Box<Primitive>, Box<Primitive>),
    IfFood(Direction, Box<Primitive>, Box<Primitive>),
    Move(Direction),
}

impl Tree for Primitive {
    const TERMINAL_PROPORTION: f32 = 0.1 / (0.1 + 0.2);

    fn arbitrary_terminal<G: Gen>(g: &mut G, _: &TreeGen, _: usize) -> Primitive {
        Primitive::Move(Direction::arbitrary(g))
    }

    fn arbitrary_nonterminal<G: Gen>(g: &mut G, t: &TreeGen, depth: usize) -> Primitive {
        match g.next_u32() % 2 {
            0 => Primitive::arbitrary_if_danger(g, t, depth),
            1 => Primitive::arbitrary_if_food(g, t, depth),
            _ => unreachable!(),
        }
    }
}

impl Primitive {
    fn arbitrary_if_danger<G: Gen>(g: &mut G, t: &TreeGen, depth: usize) -> Primitive {
        let direction = Direction::arbitrary(g);
        let true_ = Primitive::arbitrary_node(g, t, depth + 1);
        let false_ = Primitive::arbitrary_node(g, t, depth + 1);
        Primitive::IfDanger(direction, box true_, box false_)
    }

    fn arbitrary_if_food<G: Gen>(g: &mut G, t: &TreeGen, depth: usize) -> Primitive {
        let direction = Direction::arbitrary(g);
        let true_ = Primitive::arbitrary_node(g, t, depth + 1);
        let false_ = Primitive::arbitrary_node(g, t, depth + 1);
        Primitive::IfFood(direction, box true_, box false_)
    }
}

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

    fn arbitrary_terminal<G: Gen>(tg: &mut TreeGen<G>, _: usize) -> Primitive {
        Primitive::Move(Direction::arbitrary(tg.gen))
    }

    fn arbitrary_nonterminal<G: Gen>(tg: &mut TreeGen<G>, current_depth: usize) -> Primitive {
        match tg.gen.next_u32() % 2 {
            0 => Primitive::arbitrary_if_danger(tg, current_depth),
            1 => Primitive::arbitrary_if_food(tg, current_depth),
            _ => unreachable!(),
        }
    }
}

impl Primitive {
    fn arbitrary_if_danger<G: Gen>(tg: &mut TreeGen<G>, current_depth: usize) -> Primitive {
        let direction = Direction::arbitrary(tg.gen);
        let true_ = Primitive::arbitrary_node(tg, current_depth + 1);
        let false_ = Primitive::arbitrary_node(tg, current_depth + 1);
        Primitive::IfDanger(direction, box true_, box false_)
    }

    fn arbitrary_if_food<G: Gen>(tg: &mut TreeGen<G>, current_depth: usize) -> Primitive {
        let direction = Direction::arbitrary(tg.gen);
        let true_ = Primitive::arbitrary_node(tg, current_depth + 1);
        let false_ = Primitive::arbitrary_node(tg, current_depth + 1);
        Primitive::IfFood(direction, box true_, box false_)
    }
}

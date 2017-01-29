use rand::{Rng, Rand};

use tree::*;

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
pub enum Primitive {
    IfDanger(Direction, Box<Primitive>, Box<Primitive>),
    IfFood(Direction, Box<Primitive>, Box<Primitive>),
    Move(Direction),
}

impl Tree for Primitive {
    const TERMINAL_PROPORTION: f32 = 0.1 / (0.1 + 0.2);

    fn rand_terminal<R: Rng>(tg: &mut TreeGen<R>, _: usize) -> Primitive {
        Primitive::Move(Direction::rand(tg))
    }

    fn rand_nonterminal<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Primitive {
        match tg.next_u32() % 2 {
            0 => Primitive::rand_if_danger(tg, current_depth),
            1 => Primitive::rand_if_food(tg, current_depth),
            _ => unreachable!(),
        }
    }
}

impl Primitive {
    fn rand_if_danger<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Primitive {
        let direction = Direction::rand(tg);
        let true_ = Primitive::rand_node(tg, current_depth + 1);
        let false_ = Primitive::rand_node(tg, current_depth + 1);
        Primitive::IfDanger(direction, box true_, box false_)
    }

    fn rand_if_food<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> Primitive {
        let direction = Direction::rand(tg);
        let true_ = Primitive::rand_node(tg, current_depth + 1);
        let false_ = Primitive::rand_node(tg, current_depth + 1);
        Primitive::IfFood(direction, box true_, box false_)
    }
}

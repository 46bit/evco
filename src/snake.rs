use quickcheck::{Gen, Arbitrary};

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

impl Primitive {
    fn arbitrary_with_max_depth<G: Gen>(g: &mut G, max_depth: usize) -> Primitive {
        Primitive::arbitrary_with_depth_range(g, 0, max_depth)
    }

    fn arbitrary_with_depth_range<G: Gen>(g: &mut G,
                                          mut min_depth: usize,
                                          mut max_depth: usize)
                                          -> Primitive {
        if max_depth == 0 {
            return Primitive::Move(Direction::arbitrary(g));
        }
        if min_depth > 0 {
            min_depth -= 1;
        }
        max_depth -= 1;

        match g.next_u32() % 3 {
            0 => Primitive::arbitrary_if_danger(g, min_depth, max_depth),
            1 => Primitive::arbitrary_if_food(g, min_depth, max_depth),
            2 => Primitive::Move(Direction::arbitrary(g)),
            _ => unreachable!(),
        }
    }

    fn arbitrary_if_danger<G: Gen>(g: &mut G, min_depth: usize, max_depth: usize) -> Primitive {
        let max_depths = (g.gen_range(min_depth, max_depth), g.gen_range(min_depth, max_depth));
        let true_primitive = Primitive::arbitrary_with_depth_range(g, min_depth, max_depths.0);
        let false_primitive = Primitive::arbitrary_with_depth_range(g, min_depth, max_depths.1);
        Primitive::IfDanger(Direction::arbitrary(g),
                            box true_primitive,
                            box false_primitive)
    }

    fn arbitrary_if_food<G: Gen>(g: &mut G, min_depth: usize, max_depth: usize) -> Primitive {
        let max_depths = (g.gen_range(min_depth, max_depth), g.gen_range(min_depth, max_depth));
        let true_primitive = Primitive::arbitrary_with_depth_range(g, min_depth, max_depths.0);
        let false_primitive = Primitive::arbitrary_with_depth_range(g, min_depth, max_depths.1);
        Primitive::IfFood(Direction::arbitrary(g),
                          box true_primitive,
                          box false_primitive)
    }
}

impl Arbitrary for Primitive {
    fn arbitrary<G: Gen>(g: &mut G) -> Primitive {
        let size = g.size();
        let max_depth = g.gen_range(0, size);
        return Primitive::arbitrary_with_max_depth(g, max_depth);
    }
}

// 1.

pub enum Node<N> {
    Branch(usize, N),
    Leaf(usize, N),
}

////// ////// ////// ////// ////// ////// ////// //////

// 2.

pub enum SnakeBranches {
    IfDanger(),
    IfFood,
}

pub SnakeLeaves {
    Move(Direction),
}

pub enum SnakePrimitives {
    IfDanger(IfDanger),
    IfFood(IfFood),
}

pub struct IfDanger();

impl Node<T> for IfDanger where T: 'static + Tree {
    type Children<T> = (Direction, T, T);

    fn new(current_depth: usize) -> IfDanger {
        IfDanger()
    }

    fn evaluate() -> Node {
        Tree::Branch(IfFood::new())
        Tree::Leaf(Move::rand())
    }
}

pub struct IfDanger();

impl Node<T> for IfDanger where T: 'static + Tree {
    type Children<T> = (Direction, T, T);

    fn new(current_depth: usize) -> (Self, Self::Children) {

    }
}

pub enum SnakeTerminals {
    Move(Direction),
}

////// ////// ////// ////// ////// ////// ////// //////

// 3.

enum Nodes {

}

enum EvaluateResult<T: Tree> {
    Next(Vec<T>),
    Action(T::Action)
}

struct IsDanger;

impl Primitive for IsDanger {
    type Children = (SnakeTree, SnakeTree, SnakeTree);

    fn new() -> SnakeTree {
        SnakeTree::IsDanger(IsDanger)
    }

    fn evaluate(&mut self, children: &mut Self::Children, env: SnakeTree::Environment) -> EvaluateResult<SnakeTree> {
        if env.sense_danger(children.0) {
            vec![children.1]
        } else {
            vec![children.2]
        }
    }
}

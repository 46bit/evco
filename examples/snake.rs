extern crate rand;
extern crate jeepers;

use rand::{OsRng, Rng, Rand};

use jeepers::tree::{Tree, TreeGen};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TurnDirection {
    Left,
    Ahead,
    Right,
}

impl Rand for TurnDirection {
    fn rand<R: Rng>(r: &mut R) -> TurnDirection {
        match r.next_u32() % 3 {
            0 => TurnDirection::Left,
            1 => TurnDirection::Ahead,
            2 => TurnDirection::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CompassDirection {
    North,
    East,
    South,
    West,
}

impl CompassDirection {
    fn variants() -> &'static [CompassDirection] {
        static VARIANTS: &'static [CompassDirection] = &[CompassDirection::North,
                                                         CompassDirection::East,
                                                         CompassDirection::South,
                                                         CompassDirection::West];
        VARIANTS
    }
}

impl Rand for CompassDirection {
    fn rand<R: Rng>(r: &mut R) -> CompassDirection {
        match r.next_u32() % 4 {
            0 => CompassDirection::North,
            1 => CompassDirection::East,
            2 => CompassDirection::South,
            3 => CompassDirection::West,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn neighbour(&self, direction: &CompassDirection) -> Vector {
        match *direction {
            CompassDirection::North => {
                Vector {
                    x: self.x,
                    y: self.y - 1,
                }
            }
            CompassDirection::East => {
                Vector {
                    x: self.x + 1,
                    y: self.y,
                }
            }
            CompassDirection::South => {
                Vector {
                    x: self.x,
                    y: self.y + 1,
                }
            }
            CompassDirection::West => {
                Vector {
                    x: self.x - 1,
                    y: self.y,
                }
            }
        }
    }

    fn direction_to(&self, possible_neighbour: Vector) -> Option<CompassDirection> {
        for direction in CompassDirection::variants() {
            let neighbour = self.neighbour(direction);
            if neighbour == possible_neighbour {
                return Some(*direction);
            }
        }
        None
    }
}

pub struct SnakeEnvironment {
    pub size: Vector,
    pub food: Vector,
    pub snake: Vec<Vector>,
}

impl SnakeEnvironment {
    fn turn_to_compass_direction(&self, turn_direction: TurnDirection) -> CompassDirection {
        let snake_current_compass_direction = self.snake[1].direction_to(self.snake[0]).unwrap();
        let directions = CompassDirection::variants();
        let index = directions.iter().position(|&r| r == snake_current_compass_direction).unwrap();
        let new_compass_direction_index = match turn_direction {
            TurnDirection::Left => (index - 1) % 4,
            TurnDirection::Ahead => index,
            TurnDirection::Right => (index + 1) % 4,
        };
        directions[new_compass_direction_index]
    }

    fn sense_danger(&self, turn_direction: TurnDirection) -> bool {
        let compass_direction = self.turn_to_compass_direction(turn_direction);
        let cell_in_direction = self.snake[0].neighbour(&compass_direction);
        (cell_in_direction.x < 0 || cell_in_direction.y < 0 ||
         cell_in_direction.x >= self.size.x || cell_in_direction.y >= self.size.y ||
         self.snake.contains(&cell_in_direction))
    }

    fn sense_food(&self, turn_direction: TurnDirection) -> bool {
        let compass_direction = self.turn_to_compass_direction(turn_direction);
        let cell_in_direction = self.snake[0].neighbour(&compass_direction);
        self.food == cell_in_direction
    }
}

#[derive(Clone, Debug)]
pub enum SnakeTree {
    IfDanger(TurnDirection, Box<SnakeTree>, Box<SnakeTree>),
    IfFood(TurnDirection, Box<SnakeTree>, Box<SnakeTree>),
    Move(TurnDirection),
}

impl<'a> Tree<'a> for SnakeTree {
    type Environment = &'a mut SnakeEnvironment;
    type Action = TurnDirection;

    fn terminal_proportion<R: Rng>(_: &mut TreeGen<R>) -> f32 {
        1.0 / (1.0 + 2.0)
    }

    fn rand_terminal<R: Rng>(tg: &mut TreeGen<R>, _: usize) -> SnakeTree {
        SnakeTree::Move(TurnDirection::rand(tg))
    }

    fn rand_nonterminal<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> SnakeTree {
        // A list of nonterminal construction methods.
        let nonterminal_fs = [SnakeTree::rand_if_danger, SnakeTree::rand_if_food];
        // Picks a random nonterminal constructor and runs it.
        let nonterminal_f = tg.choose(&nonterminal_fs).unwrap();
        nonterminal_f(tg, current_depth)
    }

    fn evaluate(&self, env: &mut SnakeEnvironment) -> Self::Action {
        match *self {
            SnakeTree::IfDanger(direction, ref left_, ref right_) => {
                if env.sense_danger(direction) {
                    left_.evaluate(env)
                } else {
                    right_.evaluate(env)
                }
            }
            SnakeTree::IfFood(direction, ref left_, ref right_) => {
                if env.sense_food(direction) {
                    left_.evaluate(env)
                } else {
                    right_.evaluate(env)
                }
            }
            SnakeTree::Move(direction) => direction,
        }
    }
}

impl SnakeTree {
    fn rand_if_danger<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> SnakeTree {
        let direction = TurnDirection::rand(tg);
        let true_ = SnakeTree::rand_node(tg, current_depth + 1);
        let false_ = SnakeTree::rand_node(tg, current_depth + 1);
        SnakeTree::IfDanger(direction, Box::new(true_), Box::new(false_))
    }

    fn rand_if_food<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> SnakeTree {
        let direction = TurnDirection::rand(tg);
        let true_ = SnakeTree::rand_node(tg, current_depth + 1);
        let false_ = SnakeTree::rand_node(tg, current_depth + 1);
        SnakeTree::IfFood(direction, Box::new(true_), Box::new(false_))
    }
}

fn main() {
    let mut rng = OsRng::new().unwrap();

    let d = TurnDirection::rand(&mut rng);
    println!("{:?}", d);

    let mut tree_gen = TreeGen::full(&mut rng, 5, 10);
    let p = SnakeTree::rand_tree(&mut tree_gen);
    println!("{:?}", p);
}
